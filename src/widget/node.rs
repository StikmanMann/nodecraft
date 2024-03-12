use iced::{
    advanced::{
        layout, renderer,
        widget::{self, tree, Tree},
        Widget,
    },
    widget::{text, text_input::DEFAULT_PADDING},
    Border, Color, Element, Length, Padding, Size,
};

pub struct Node<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    inputs: Vec<String>,
    outputs: Vec<String>,
    padding: Padding,
    content: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> Node<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Craetes a new [´Node´] with the give inputs and outputs
    pub fn new(
        inputs: Vec<String>,
        outputs: Vec<String>,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            inputs,
            outputs,
            padding: DEFAULT_PADDING,
            content: content.into(),
        }
    }

    /// Sets the [`Padding`] of the Node
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }
}

pub fn node<'a, Message, Theme, Renderer>(
    inputs: Vec<String>,
    outputs: Vec<String>,
    content: Element<'a, Message, Theme, Renderer>,
) -> Node<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    Node::new(inputs, outputs, content)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Node<'a, Message, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer,
    Message: 'a + Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut state.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            )
        })
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::padded(
            limits,
            Length::Shrink,
            Length::Shrink,
            self.padding,
            |limits| {
                self.content
                    .as_widget()
                    .layout(&mut tree.children[0], renderer, limits)
            },
        )
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border::with_radius(10),
                ..renderer::Quad::default()
            },
            Color::from_rgba(0.1, 0.6, 0.3, 1.0),
        );
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Node<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: 'a + renderer::Renderer,
{
    fn from(node: Node<'a, Message, Theme, Renderer>) -> Self {
        Self::new(node)
    }
}
