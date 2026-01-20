#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PieceId(pub char);

#[derive(Clone, Debug)]
pub struct Orientation {
    pub cells: Vec<(i32, i32)>,
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub id: PieceId,
    pub orientations: Vec<Orientation>,
}

