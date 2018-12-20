use super::Board;

use crate::chess::bitboard::FILES;
use crate::chess::{BitBoard, BitPosition, PieceType, Player, RankFile};

impl Board {
    // TODO: King vs King check
    pub fn is_attacked(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> bool {
        let rook_moves = self.find_rook_moves(current_position, current_position_mask);

        let queen_rook_threats = rook_moves
            .intersect(
                self.pieces[PieceType::Rook as usize].join(self.pieces[PieceType::Queen as usize]),
            )
            .intersect(self.enemy_mask());

        println!("King:\n{:?}", current_position_mask);
        println!("King Rook Moves:\n{:?}", rook_moves);
        println!("{:?}", queen_rook_threats);

        if !queen_rook_threats.is_empty() {
            covered_by!("MoveGenerator::rook_attacks");
            return true;
        }

        let knight_threats = self
            .find_knight_moves(current_position, current_position_mask)
            .intersect(self.pieces[PieceType::Knight as usize])
            .intersect(self.enemy_mask());

        if !knight_threats.is_empty() {
            covered_by!("MoveGenerator::knight_attacks");
            return true;
        }

        let diagonals = self.find_bishop_moves(current_position, current_position_mask);

        let queen_bishop_threats = diagonals
            .intersect(
                self.pieces[PieceType::Bishop as usize]
                    .join(self.pieces[PieceType::Queen as usize]),
            )
            .intersect(self.enemy_mask());

        if !queen_bishop_threats.is_empty() {
            covered_by!("MoveGenerator::bishop_attacks");
            return true;
        }

        let pawn_threats =
            diagonals.intersect(self.pieces[PieceType::Pawn as usize].intersect(self.enemy_mask()));

        if !pawn_threats.is_empty() {
            let rank = RankFile::from(current_position).rank();

            match self.next_player {
                Player::White => {
                    if rank < 7
                        && !FILES[(rank + 1) as usize]
                            .intersect(pawn_threats)
                            .is_empty()
                    {
                        covered_by!("MoveGenerator::black_pawn_attacks");
                        return true;
                    }
                }
                Player::Black => {
                    if rank > 0
                        && !FILES[(rank - 1) as usize]
                            .intersect(pawn_threats)
                            .is_empty()
                    {
                        covered_by!("MoveGenerator::white_pawn_attacks");
                        return true;
                    }
                }
            };
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::Board;
    use crate::fixtures::*;

    fn check_spaces(
        board: &str,
        player: Player,
        attacked_spaces: &[RankFile],
        safe_spaces: &[RankFile],
    ) {
        let board = Board::from(board, player).unwrap();

        for &space in attacked_spaces.iter() {
            assert_eq!(
                board.is_attacked(space.into(), space.into()),
                true,
                "Expected {:?} to be attacked",
                space
            );
        }

        for &space in safe_spaces.iter() {
            assert_eq!(
                board.is_attacked(space.into(), space.into()),
                false,
                "Expected {:?} to be safe",
                space
            );
        }
    }
    // TODO: Test Queen, Pawn, King

    #[test]
    fn test_rook_attacks() {
        covers!("MoveGenerator::rook_attacks");

        let attacked_spaces = [
            RankFile::A1,
            RankFile::A7,
            RankFile::B1,
            RankFile::E2,
            RankFile::H5,
        ];

        let safe_spaces = [
            RankFile::B3,
            RankFile::H6,
            RankFile::G8,
            RankFile::F2,
            RankFile::B6,
        ];

        check_spaces(
            WHITE_ROOK_TEST,
            Player::Black,
            &attacked_spaces,
            &safe_spaces,
        );
    }

    #[test]
    fn test_bishop_attacks() {
        covers!("MoveGenerator::bishop_attacks");

        let attacked_spaces = [
            RankFile::B7,
            RankFile::C2,
            RankFile::G2,
            RankFile::G8,
            RankFile::F3,
        ];

        let safe_spaces = [
            RankFile::H2,
            RankFile::A4,
            RankFile::A1,
            RankFile::B4,
            RankFile::G7,
        ];

        check_spaces(
            WHITE_BISHOP_TEST,
            Player::Black,
            &attacked_spaces,
            &safe_spaces,
        );
    }

    #[test]
    fn test_knight_attacks() {
        covers!("MoveGenerator::knight_attacks");

        let attacked_spaces = [
            RankFile::D2,
            RankFile::F2,
            RankFile::C3,
            RankFile::B4,
            RankFile::F6,
        ];

        let safe_spaces = [
            RankFile::H2,
            RankFile::A4,
            RankFile::A1,
            RankFile::B8,
            RankFile::H5,
        ];

        check_spaces(
            WHITE_KNIGHT_TEST,
            Player::Black,
            &attacked_spaces,
            &safe_spaces,
        );
    }

    #[test]
    fn test_white_pawn_attacks() {
        covers!("MoveGenerator::white_pawn_attacks");

        let attacked_spaces = [
            RankFile::G3,
            RankFile::A3,
            RankFile::B4,
            RankFile::D4,
            RankFile::B8,
        ];

        let safe_spaces = [
            RankFile::C1,
            RankFile::G1,
            RankFile::A1,
            RankFile::G7,
            RankFile::C4,
        ];

        check_spaces(
            WHITE_PAWN_TEST,
            Player::Black,
            &attacked_spaces,
            &safe_spaces,
        );
    }

    #[test]
    fn test_black_pawn_attacks() {
        covers!("MoveGenerator::black_pawn_attacks");

        let attacked_spaces = [
            RankFile::B6,
            RankFile::H6,
            RankFile::B4,
            RankFile::F3,
            RankFile::D1,
        ];

        let safe_spaces = [
            RankFile::C1,
            RankFile::B8,
            RankFile::B5,
            RankFile::B7,
            RankFile::E1,
        ];

        check_spaces(
            BLACK_PAWN_TEST,
            Player::White,
            &attacked_spaces,
            &safe_spaces,
        );
    }

}
