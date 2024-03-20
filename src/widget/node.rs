use iced::{
    advanced::{
        graphics::text::cosmic_text::rustybuzz::ttf_parser::name, layout, renderer::{self, Quad}, widget::{self, tree, Tree}, Widget
    },
    widget::{rule::FillMode, text, text_input::DEFAULT_PADDING},
    Border, Color, Element, Length, Padding, Size,
};
pub struct Node {
    pub label: String,
    pub inputs: Vec<Parameter>,
    pub outputs: Vec<Parameter>,
}
impl Node {
    pub fn new(lable : String, inputs : Vec<Parameter>, outputs : Vec<Parameter>) -> Self {
        Self {
            label: String::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
    
}
impl<Message, Renderer, Theme> Widget<Message, Theme, Renderer> for Node
    where Renderer : renderer::Renderer{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new([100, 100].into())
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        
        renderer.fill_quad(
            Quad{
            bounds: layout.bounds(),
            ..Default::default()
        },
         Color::from_rgb(0.0, 0.0, 0.0)
        );
        iced::widget::text::draw(renderer, style, layout, state, appearance, viewport)
    }
}
pub struct Parameter {
    pub data_type: EDatarTypes,
    pub name: String,
}

pub enum EDatarTypes {
    Number,
    String,
    Boolean,
}

impl<'a, Message, Theme, Renderer> From<Node>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: 'a + renderer::Renderer,
{
    fn from(node: Node) -> Self {
        Self::new(node)
    }
}