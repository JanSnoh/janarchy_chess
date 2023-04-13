use crate::backend;
use crate::backend::pieces::PieceColor;
impl backend::GameState {
    pub(crate) fn prnt(&self, perspective: Option<PieceColor>) {
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
                    backend::pieces::Piece::to_char(&self.fields[i * 8 + j])
                );
            }
            println!("{}", s);
        }
    }
}
