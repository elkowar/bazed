export const websocket = async () => {
  try {
    const ws = new WebSocket("ws://localhost:6969")
    const cmd = {
      method: "key_pressed",
      params: {
        key: "X",
        modifiers: [],
      },
    }

    const _ = await new Promise((resolve) => {
      ws.onopen = (event) => resolve(event)
    })

    const openDocumentEvt: OpenDocument = await new Promise((resolve) => {
      ws.onmessage = (event) => resolve(JSON.parse(event.data))
    })

    console.log(openDocumentEvt)

    const openViewMsg: ViewOpened = {
      method: "view_opened",
      params: {
        request_id: "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d", // "random" uuid
        document_id: openDocumentEvt.params.document_id,
        height: 20,
        width: 40,
      },
    }
    ws.send(JSON.stringify(openViewMsg))
    // reassigning onmessage constantly is definitely not the strat, this is pain
    // we need a good abstraction here lmao (as in, we're doing RPC, not random haha let's throw data around the interwebs)
    const viewOpenedResponse: ViewOpenedResponse = await new Promise((resolve) => {
      ws.onmessage = (event) => resolve(JSON.parse(event.data))
    })

    console.log(viewOpenedResponse)
    const poggersViewId = viewOpenedResponse.params.view_id
    console.log(poggersViewId)

    const handlers: Handlers = {
      open_document: (params) => {
        // do stuff in ui
      },
      view_opened_response: (params) => {
        // do stuff in ui
      },
      update_view: (params) => {
        // do stuff in ui
        console.log("Call some funny callback from the respective widget or store or sth")
        console.log(params.text)
      },
    }

    const helper = (obj: any) => ws.send(JSON.stringify(obj))

    const wsClient = {}

    const _wsClient = {
      key_pressed: (params: KeyPressed["params"]) => {
        ws.send(JSON.stringify({ method: "key_pressed", params }))
      },
    }

    ws.onmessage = (event) => {
      const command: ToFrontend = JSON.parse(event.data)
      handlers[command.method](command.params as any)
    }

    ws.send(JSON.stringify(cmd))
  } catch (e) {
    console.log(e)
  }
}

type Uuid = string
type RequestId = string

type Position = {
  line: number
  col: number
}

type Handlers = { [T in ToFrontend as T["method"]]: (params: T["params"]) => void }

type Message<Method extends string, Params> = {
  method: Method
  params: Params
}

type ToFrontend = OpenDocument | UpdateView | ViewOpenedResponse

const to_backend = <const>[
  "view_opened",
  "viewport_changed",
  "key_pressed",
  "save_document",
  "mouse_input",
]
type ToBackend = ViewOpened | ViewportChanged | KeyPressed | SaveDocument | MouseInput

type OpenDocument = Message<
  "open_document",
  {
    document_id: Uuid
    path: string | null
    text: string
  }
>

type UpdateView = Message<
  "update_view",
  {
    view_id: Uuid
    first_line: number
    height: number
    text: string[]
    carets: Position[]
  }
>

type ViewOpenedResponse = Message<
  "view_opened_response",
  {
    request_id: RequestId
    view_id: Uuid
  }
>

type ViewOpened = Message<
  "view_opened",
  {
    request_id: RequestId
    document_id: Uuid
    height: number
    width: number
  }
>

type ViewportChanged = Message<
  "viewport_changed",
  {
    view_id: Uuid
    height: number
    width: number
    first_line: number
    first_col: number
  }
>

type KeyPressed = Message<
  "key_pressed",
  {
    view_id: Uuid
    position: Position
  }
>

type SaveDocument = Message<
  "save_document",
  {
    document_id: Uuid
  }
>

// backend unimplemented
type MouseInput = Message<
  "mouse_input",
  {
    view_id: Uuid
    position: Position
  }
>
