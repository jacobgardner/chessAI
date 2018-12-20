use super::MoveGenerator;

use crate::chess::bitboard::RANKS;
use crate::chess::{BitBoard, BitPosition, PieceType, Player, RankFile};

impl MoveGenerator {
    pub fn is_attacked(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> bool {
        let queen_rook_threats = self
            .find_rook_moves(current_position, current_position_mask)
            .intersect(
                self.root_board.pieces[PieceType::Rook as usize]
                    .join(self.root_board.pieces[PieceType::Queen as usize]),
            )
            .intersect(self.enemy_mask);

        if !queen_rook_threats.is_empty() {
            covered_by!("MoveGenerator::rook_attacks");
            return true;
        }

        let knight_threats = self
            .find_knight_moves(current_position, current_position_mask)
            .intersect(self.root_board.pieces[PieceType::Knight as usize])
            .intersect(self.enemy_mask);

        if !knight_threats.is_empty() {
            return true;
        }

        let diagonals = self.find_bishop_moves(current_position, current_position_mask);

        let queen_bishop_threats = diagonals
            .intersect(
                self.root_board.pieces[PieceType::Bishop as usize]
                    .join(self.root_board.pieces[PieceType::Queen as usize]),
            )
            .intersect(self.enemy_mask);

        println!("{:?}", queen_bishop_threats);

        if !queen_bishop_threats.is_empty() {
            covered_by!("MoveGenerator::bishop_attacks");
            return true;
        }

        let pawn_threats = diagonals
            .intersect(self.root_board.pieces[PieceType::Pawn as usize].intersect(self.enemy_mask));

        if !pawn_threats.is_empty() {
            let rank = RankFile::from(current_position).rank();

            match self.player {
                Player::White => {
                    if rank < 7 {
                        if !RANKS[(rank + 1) as usize]
                            .intersect(pawn_threats)
                            .is_empty()
                        {
                            return true;
                        }
                    }
                }
                Player::Black => {
                    if rank > 0 {
                        if !RANKS[(rank - 1) as usize]
                            .intersect(pawn_threats)
                            .is_empty()
                        {
                            return true;
                        }
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
        let board = Board::from(board).unwrap();
        let generator = MoveGenerator::new(board, player);

        for &space in attacked_spaces.iter() {
            assert_eq!(generator.is_attacked(space.into(), space.into()), true, "Expected {:?} to be attacked", space);
        }

        for &space in safe_spaces.iter() {
            assert_eq!(generator.is_attacked(space.into(), space.into()), false, "Expected {:?} to be safe", space);
        }
    }

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

        check_spaces(WHITE_ROOK_TEST, Player::Black, &attacked_spaces, &safe_spaces);
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

        check_spaces(WHITE_BISHOP_TEST, Player::Black, &attacked_spaces, &safe_spaces);
    }


}
