use std::{collections::HashMap, sync::Arc};

use bazed_rpc::{core_proto::ToBackend, core_proto::ToFrontend, server::ClientSendHandle};
use color_eyre::Result;
use futures::StreamExt;
use tokio::sync::RwLock;

use crate::document::{Document, DocumentId};

pub struct App {
    documents: HashMap<DocumentId, Document>,
    active_document: Option<DocumentId>,
    event_send: ClientSendHandle,
}

impl App {
    pub fn new(event_send: ClientSendHandle) -> Self {
        App {
            documents: HashMap::new(),
            active_document: None,
            event_send,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn open_ephemeral(&mut self) -> Result<()> {
        let document = Document::open_ephemeral();
        let id = DocumentId::gen();
        self.event_send
            .send_rpc_notification(ToFrontend::Open {
                id: id.0,
                title: document.title.clone(),
                text: document.buffer.content_to_string(),
            })
            .await?;
        self.documents.insert(id, document);
        self.active_document = Some(id);
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn handle_rpc_call(&mut self, call: ToBackend) -> Result<()> {
        tracing::info!(call = ?call, "Handling rpc call");
        match call {
            ToBackend::KeyPressed(key) => {
                let Some(document_id) = self.active_document else { return Ok(()) };
                let Some(ref mut document) = self.documents.get_mut(&document_id) else { return Ok(()) };

                if let Some(c) = key.key.try_to_char() {
                    document.buffer.insert_at_primary(&c.to_string());
                    self.event_send
                        .send_rpc_notification(ToFrontend::UpdateText {
                            id: document_id.0,
                            text: document.buffer.content_to_string(),
                        })
                        .await?;
                }
            },
            ToBackend::MouseInput { line, column } => {
                tracing::info!("mouse input: {column},{line}")
            },
        }
        Ok(())
    }
}

pub async fn start(addr: &str) -> Result<()> {
    let (send, mut recv) = bazed_rpc::server::wait_for_client(addr).await?;

    let core = Arc::new(RwLock::new(App::new(send)));

    tokio::spawn({
        let core = core.clone();
        async move {
            while let Some(rpc_call) = recv.next().await {
                let mut core = core.write().await;
                if let Err(err) = core.handle_rpc_call(rpc_call).await {
                    tracing::error!("Failed to handle rpc call: {err:?}");
                }
            }
        }
    });

    core.write().await.open_ephemeral().await?;

    Ok(())
}
