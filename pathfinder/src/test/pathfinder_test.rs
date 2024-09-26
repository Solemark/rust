#[cfg(test)]
mod tests {
    use crate::Pathfinder;
    const BOARD: [[char; 10]; 10] = [
        ['@', '@', '@', '@', '@', '@', '@', '@', '@', '@'],
        ['@', 'X', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@'],
        ['@', '@', '@', '@', '@', ' ', '@', '@', ' ', '@'],
        ['@', ' ', ' ', ' ', '@', ' ', ' ', '@', ' ', '@'],
        ['@', ' ', '@', ' ', '@', ' ', ' ', '@', ' ', '@'],
        ['@', ' ', '@', ' ', '@', '@', '@', '@', ' ', '@'],
        ['@', ' ', '@', ' ', ' ', ' ', ' ', ' ', ' ', '@'],
        ['@', ' ', '@', '@', '@', ' ', '@', '@', '@', '@'],
        ['@', ' ', ' ', ' ', '@', ' ', ' ', ' ', '+', '@'],
        ['@', '@', '@', '@', '@', '@', '@', '@', '@', '@'],
    ];
    const COMPLETE: [[char; 10]; 10] = [
        ['@', '@', '@', '@', '@', '@', '@', '@', '@', '@'],
        ['@', 'X', '+', '+', '+', '+', '+', '+', '+', '@'],
        ['@', '@', '@', '@', '@', ' ', '@', '@', '+', '@'],
        ['@', ' ', ' ', ' ', '@', ' ', ' ', '@', '+', '@'],
        ['@', ' ', '@', ' ', '@', ' ', ' ', '@', '+', '@'],
        ['@', ' ', '@', ' ', '@', '@', '@', '@', '+', '@'],
        ['@', ' ', '@', ' ', ' ', '+', '+', '+', '+', '@'],
        ['@', ' ', '@', '@', '@', '+', '@', '@', '@', '@'],
        ['@', ' ', ' ', ' ', '@', '+', '+', '+', '+', '@'],
        ['@', '@', '@', '@', '@', '@', '@', '@', '@', '@'],
    ];

    #[test]
    fn test_draw_board() {
        let pf = Pathfinder { field: COMPLETE };
        let expect: String = String::from(
            "@@@@@@@@@@\n@X+++++++@\n@@@@@ @@+@\n@   @  @+@\n@ @ @  @+@\n@ @ @@@@+@\n@ @  ++++@\n@ @@@+@@@@\n@   @++++@\n@@@@@@@@@@\n"
        );
        let result: String = pf.draw_board();
        assert_eq!(expect, result);
    }

    #[test]
    fn test_preprocess() {
        let pf = Pathfinder { field: BOARD }.preprocess().field;
        let mut expect = BOARD;
        (expect[1][5], expect[3][5], expect[6][5]) = ('T', 'T', 'T');
        assert_eq!(expect, pf);
    }

    #[test]
    fn test_check_field() {
        let pf = Pathfinder { field: BOARD };
        assert_eq!('T', pf.check_field(1, 5));
        assert_eq!(' ', pf.check_field(2, 5));
    }
}
