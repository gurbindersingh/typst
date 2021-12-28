//! Document-structuring section headings.

use super::prelude::*;
use super::{FontFamily, TextNode};

/// A section heading.
#[derive(Debug, Hash)]
pub struct HeadingNode {
    /// The node that produces the heading's contents.
    pub child: PackedNode,
    /// The logical nesting depth of the section, starting from one. In the
    /// default style, this controls the text size of the heading.
    pub level: usize,
}

#[properties]
impl HeadingNode {
    /// The heading's font family.
    pub const FAMILY: Smart<String> = Smart::Auto;
    /// The fill color of heading in the text. Just the surrounding text color
    /// if `auto`.
    pub const FILL: Smart<Paint> = Smart::Auto;
}

impl Construct for HeadingNode {
    fn construct(_: &mut EvalContext, args: &mut Args) -> TypResult<Node> {
        Ok(Node::block(Self {
            child: args.expect::<Node>("body")?.into_block(),
            level: args.named("level")?.unwrap_or(1),
        }))
    }
}

impl Set for HeadingNode {
    fn set(args: &mut Args, styles: &mut Styles) -> TypResult<()> {
        styles.set_opt(Self::FAMILY, args.named("family")?);
        styles.set_opt(Self::FILL, args.named("fill")?);
        Ok(())
    }
}

impl Layout for HeadingNode {
    fn layout(
        &self,
        ctx: &mut LayoutContext,
        regions: &Regions,
    ) -> Vec<Constrained<Rc<Frame>>> {
        let upscale = (1.6 - 0.1 * self.level as f64).max(0.75);
        ctx.styles.set(TextNode::STRONG, true);
        ctx.styles.set(TextNode::SIZE, Relative::new(upscale).into());

        if let Smart::Custom(family) = ctx.styles.get_ref(Self::FAMILY) {
            let list: Vec<_> = std::iter::once(FontFamily::named(family))
                .chain(ctx.styles.get_ref(TextNode::FAMILY_LIST).iter().cloned())
                .collect();
            ctx.styles.set(TextNode::FAMILY_LIST, list);
        }

        if let Smart::Custom(fill) = ctx.styles.get(Self::FILL) {
            ctx.styles.set(TextNode::FILL, fill);
        }

        self.child.layout(ctx, regions)
    }
}