use yew::prelude::*;
use crate::editor::state::EditorState;
use crate::editor::syntax_highlighting::SyntaxHighlighter;
use crate::networking::peer_sync::PeerSync;
use crate::networking::websocket::WebSocketClient;

pub struct WebUI {
    link: ComponentLink<Self>,
    state: EditorState,
    syntax_highlighter: SyntaxHighlighter,
    peer_sync: PeerSync,
    websocket_client: Option<WebSocketClient>,
}

pub enum Msg {
    InputChanged(String),
    ReceiveWebSocketMessage(String),
    ApplyChanges,
}

impl Component for WebUI {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let websocket_client = WebSocketClient::new("ws://localhost:8080");
        
        Self {
            link,
            state: EditorState::new(),
            syntax_highlighter: SyntaxHighlighter::new(),
            peer_sync: PeerSync::new(),
            websocket_client: Some(websocket_client),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputChanged(input) => {
                self.state.insert_text(&input);
                self.syntax_highlighter.highlight(&mut self.state);

                if let Some(ref mut ws_client) = self.websocket_client {
                    let changes = self.state.get_text();
                    let _ = self.peer_sync.broadcast_change(&self.state, ws_client);
                }

                true
            }
            Msg::ReceiveWebSocketMessage(message) => {
                self.peer_sync.handle_incoming_message(message, &mut self.state);
                true
            }
            Msg::ApplyChanges => {
                // Render any changes received from peers and applied locally.
                self.syntax_highlighter.highlight(&mut self.state);
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="editor-container">
                <textarea
                    class="editor"
                    value={self.state.get_text().clone()}
                    oninput=self.link.callback(|e: InputData| Msg::InputChanged(e.value))
                />
                <div class="highlighted-code">
                    { self.render_highlighted_code() }
                </div>
            </div>
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(ref mut ws_client) = self.websocket_client {
            let link = self.link.clone();
            // Start listening for WebSocket messages
            wasm_bindgen_futures::spawn_local(async move {
                while let Some(message) = ws_client.receive_message().await {
                    link.send_message(Msg::ReceiveWebSocketMessage(message));
                }
            });
        }
        false
    }
}

impl WebUI {
    /// Renders the highlighted code into HTML
    fn render_highlighted_code(&self) -> Html {
        let highlighted_lines = self.state.get_highlighted_lines();

        html! {
            <pre class="code">
                { for highlighted_lines.iter().map(|line| html! { <div>{ line }</div> }) }
            </pre>
        }
    }
}
