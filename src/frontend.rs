use crate::pieces::PieceColor;
use crate::game::pieces::Piece;
use crate::game::moves::Field;

impl crate::GameState {
    pub fn print(&self, perspective: Option<PieceColor>) {
        type BoxedIter = Box<dyn Iterator<Item = usize>>;
        //define Iterator to loop through squares appropriately based on perspective
        fn iterate_perspective(perspective: Option<PieceColor>) -> BoxedIter {
            if let Some(PieceColor::Black) = perspective {
                Box::new(0..8)
            } else {
                Box::new((0..8).rev())
            }
        }

        for i in iterate_perspective(perspective) {
            let mut s = String::new();
            for j in iterate_perspective(perspective) {
                s = format!(
                    "{} {:#?}",
                    s,
                    Piece::to_char(&self[Field(j, i)])
                );
            }
            println!("{}", s);
        }
    }
}
//ctx.debug_painter().circle(egui::Pos2 { x: 400.0, y: 400.0 }, 100.0, WHITE, Stroke{ width: 5.0, color: BLACK });
