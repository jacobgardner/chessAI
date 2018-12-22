use super::Board;

use crate::chess::bitboard::FILES;
use crate::chess::{BitBoard, BitPosition, PieceType, Player, RankFile};

impl Board {
    fn single_check(
        &self,
        player: Player,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> bool {
        // TODO: Clean this up
        let enemy_mask = self.players[1 - player as usize];
        let rook_moves = self.find_rook_moves(current_position, current_position_mask);

        let queen_rook_threats = rook_moves
            .intersect(
                self.pieces[PieceType::Rook as usize].join(self.pieces[PieceType::Queen as usize]),
            )
            .intersect(enemy_mask);

        if !queen_rook_threats.is_empty() {
            covered_by!("MoveGenerator::rook_attacks");
            return true;
        }

        let knight_threats = self
            .find_knight_moves(current_position, current_position_mask)
            .intersect(self.pieces[PieceType::Knight as usize])
            .intersect(enemy_mask);

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
            .intersect(enemy_mask);

        if !queen_bishop_threats.is_empty() {
            covered_by!("MoveGenerator::bishop_attacks");
            return true;
        }

        let pawn_threats =
            diagonals.intersect(self.pieces[PieceType::Pawn as usize].intersect(enemy_mask));

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

    // TODO: King vs King check
    pub fn is_attacked(&self, player: Player, mut space_mask: BitBoard) -> bool {
        // TODO: This is a pretty common pattern... we should probably turn the bitboard
        //  into an iterator
        while !space_mask.is_empty() {
            let current_position = space_mask.first_bit_position();
            let current_position_mask = BitBoard::from(current_position);

            if self.single_check(player, current_position, current_position_mask) {
                return true;
            }

            space_mask -= current_position_mask;
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
                board.single_check(player, space.into(), space.into()),
                true,
                "Expected {:?} to be attacked",
                space
            );
        }

        for &space in safe_spaces.iter() {
            assert_eq!(
                board.single_check(player, space.into(), space.into()),
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
