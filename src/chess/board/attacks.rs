use super::Board;

use crate::chess::bitboard::FILES;
use crate::chess::{BitBoard, BitPosition, PieceType, Player, RankFile};

impl Board {
    fn rook_queen_threats(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
        enemy_mask: BitBoard,
    ) -> bool {
        let rook_moves = self.find_rook_moves(current_position, current_position_mask);
        let queen_rook_threats = rook_moves
            .intersect(
                self.pieces[PieceType::Rook as usize].join(self.pieces[PieceType::Queen as usize]),
            )
            .intersect(enemy_mask);

        !queen_rook_threats.is_empty()
    }

    fn bishop_queen_threats(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
        enemy_mask: BitBoard,
    ) -> bool {
        let diagonals = self.find_bishop_moves(current_position, current_position_mask);

        let queen_bishop_threats = diagonals
            .intersect(
                self.pieces[PieceType::Bishop as usize]
                    .join(self.pieces[PieceType::Queen as usize]),
            )
            .intersect(enemy_mask);

        !queen_bishop_threats.is_empty()
    }

    fn knight_threats(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
        enemy_mask: BitBoard,
    ) -> bool {
        let knight_threats = self
            .find_knight_moves(current_position, current_position_mask)
            .intersect(self.pieces[PieceType::Knight as usize])
            .intersect(enemy_mask);

        !knight_threats.is_empty()
    }

    fn pawn_threats(
        &self,
        player: Player,
        current_position_mask: BitBoard,
        enemy_mask: BitBoard,
    ) -> bool {
        // let pawn_threats =
        //     diagonals.intersect(self.pieces[PieceType::Pawn as usize].intersect(enemy_mask));

        let pawn_aoe = match player {
            Player::White => current_position_mask.shift(1, 1).join(current_position_mask.shift(1, -1)),
            Player::Black => current_position_mask.shift(-1, 1).join(current_position_mask.shift(-1, -1)),
        };

        !pawn_aoe
            .intersect(enemy_mask)
            .intersect(self.pieces[PieceType::Pawn as usize])
            .is_empty()
    }

    fn single_check(
        &self,
        player: Player,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> bool {
        // TODO: King vs King check
        let enemy_mask = self.players[1 - player as usize];

        if self.rook_queen_threats(current_position, current_position_mask, enemy_mask) {
            covered_by!("MoveGenerator::rook_attacks");
            return true;
        }

        if self.knight_threats(current_position, current_position_mask, enemy_mask) {
            covered_by!("MoveGenerator::knight_attacks");
            return true;
        }

        if self.bishop_queen_threats(current_position, current_position_mask, enemy_mask) {
            covered_by!("MoveGenerator::bishop_attacks");
            return true;
        }

        if self.pawn_threats(player, current_position_mask, enemy_mask) {
            covered_by!("MoveGenerator::pawn_attacks");
            return true;
        }

        false
    }

    pub fn is_attacked(&self, player: Player, mut space_mask: BitBoard) -> bool {
        space_mask
            .find(|current_position| {
                self.single_check(player, *current_position, BitBoard::from(*current_position))
            })
            .is_some()
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
        covers!("MoveGenerator::pawn_attacks");

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
        covers!("MoveGenerator::pawn_attacks");

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
