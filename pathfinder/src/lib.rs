mod test;
enum Type {
    Row,
    Column,
}
/**
 * `const R: usize` is for row
 * `const C: usize` is for column
 */
pub struct Pathfinder<const R: usize, const C: usize> {
    field: [[char; R]; C],
}
#[allow(dead_code)]
impl<const R: usize, const C: usize> Pathfinder<R, C> {
    /**
     * transform field into a [`String`]
     * param: [`self`]
     */
    fn draw_board(self) -> String {
        let mut out: String = String::new();
        for r in self.field {
            for c in r {
                out.push(c);
            }
            out.push('\n');
        }
        out
    }

    fn calculate(&self) -> Pathfinder<R, C> {
        // TODO
        let output = self.field;
        Pathfinder { field: output }
    }

    /**
     * preprocess field and return a new instance
     * param: [`self`]
     */
    fn preprocess(&self) -> Pathfinder<R, C> {
        let mut output: [[char; R]; C] = self.field;
        let mut r = 0;
        while r < R {
            let mut c = 0;
            while c < C {
                if self.field[r][c] != '@' {
                    output[r][c] = self.check_field(r, c);
                }
                c += 1;
            }
            r += 1;
        }
        Pathfinder { field: output }
    }

    /**
     * determine of current space (`self.field`) is an intersection
     * param [`usize`] r -> row
     * param [`usize`] c -> column
     * return [`char`]
     */
    fn check_field(&self, r: usize, c: usize) -> char {
        let rf = {
            let mut rf = false;
            if self.check_both(r, c, Type::Row) {
                rf = true;
            }
            rf
        };
        let cf = {
            let mut cf = false;
            if self.check_both(r, c, Type::Column) {
                cf = true;
            }
            cf
        };
        if rf || cf {
            return 'T';
        }
        self.field[r][c]
    }

    /**
     * check for a T or a + shape of free spaces `char = ' '`
     * param [`usize`] r -> row
     * param [`usize`] c -> column
     * param [`Type`] t -> are we checking [`Type::Row`] or [`Type::Column`]?
     * return [`bool`]
     */
    fn check_both(&self, r: usize, c: usize, t: Type) -> bool {
        match t {
            Type::Row => {
                (self.field[r - 1][c] != '@' && self.field[r + 1][c] != '@')
                    && (self.field[r][c - 1] != '@' || self.field[r][c + 1] != '@')
            }
            Type::Column => {
                (self.field[r][c - 1] != '@' && self.field[r][c + 1] != '@')
                    && (self.field[r - 1][c] != '@' || self.field[r + 1][c] != '@')
            }
        }
    }
}
