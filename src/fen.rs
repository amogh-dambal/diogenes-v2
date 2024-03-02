use super::types::Piece;
use super::types::Color;
use super::types::NUM_COLORS;
use super::square::Square;

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn get_piece(c: char) -> (Piece, Color) {
    match c {
        'p' => {(Piece::Pawn, Color::Black)}
        'n' => {(Piece::Knight, Color::Black)}
        'b' => {(Piece::Bishop, Color::Black)}
        'r' => {(Piece::Rook, Color::Black)}
        'q' => {(Piece::Queen, Color::Black)}
        'k' => {(Piece::King, Color::Black)}
        'P' => {(Piece::Pawn, Color::White)}
        'N' => {(Piece::Knight, Color::White)}
        'B' => {(Piece::Bishop, Color::White)}
        'R' => {(Piece::Rook, Color::White)}
        'Q' => {(Piece::Queen, Color::White)}
        'K' => {(Piece::King, Color::White)}
        _ => {panic!("Invalid string in FEN")}
    }
}

// Parse the first field in a (valid) FEN string and return the
// piece bitboards that represent this position.
pub fn parse_pieces(field: &str) -> [u64; 8] {
    // In FEN notation, the 8th rank is written first. Since we want to 
    // populate ranks in ascending order, we need to reverse this iterator.
    let ranks = field.rsplit("/");
    let mut piece_bbs: [u64; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    for (r, pieces) in ranks.enumerate() {
        let mut f = 0;
        for piece in pieces.chars() {
            match piece.to_digit(10) {
                Some(skip) => {f += skip},
                None => {
                    let (p, c) = get_piece(piece);
                    let bb: u64 = 1 << Square::index(f as u32, r as u32);
                    piece_bbs[c as usize] |= bb;
                    piece_bbs[(NUM_COLORS as usize) + (p as usize)] |= bb;
                }
            }
        }
    }

    return piece_bbs;
}

#[cfg(test)]
mod tests {
    use std::char;

    use super::get_piece;
    use super::Piece;
    use super::Color;

    #[test]
    fn test_piece_char_match() {
        let black_piece_chars: [char; 6] = ['p', 'n', 'b', 'r', 'q', 'k'];
        let pieces: [Piece; 6] = [Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King];
        let mut color: Color = Color::Black;

        for (i, char) in black_piece_chars.iter().enumerate() {
            let (p, c) = get_piece(*char);
            assert_eq!(p, pieces[i]);
            assert_eq!(c, color);
        }

        color = Color::White;
        let white_piece_chars: [char; 6] = ['P', 'N', 'B', 'R', 'Q', 'K'];
        for (i, char) in white_piece_chars.iter().enumerate() {
            let (p, c) = get_piece(*char);
            assert_eq!(p, pieces[i]);
            assert_eq!(c, color);
        }
    }
}