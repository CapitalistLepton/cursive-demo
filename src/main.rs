use cursive::views::Canvas;
use cursive::{Cursive, CursiveExt, XY};

const MAP_W: usize = 15;
const MAP_H: usize = 10;

#[derive(Copy, Clone)]
enum Tile {
    Blank,
    Wall,
}

struct GameState {
    tiles: [Tile; MAP_W * MAP_H],
}

impl GameState {
    fn new() -> Self {
        let mut state = Self {
            tiles: [Tile::Blank; MAP_W * MAP_H],
        };
        for y in 0..MAP_H {
            for x in 0..MAP_W {
                if y == 0 || y == MAP_H - 1 {
                    state.tiles[y * MAP_W + x] = Tile::Wall;
                } else if x == 0 || x == MAP_W - 1 {
                    state.tiles[y * MAP_W + x] = Tile::Wall;
                }
            }
        }
        state
    }
}

fn main() {
    let mut siv = Cursive::new();
    siv.add_layer(create_canvas());
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
}

fn create_canvas() -> Canvas<GameState> {
    Canvas::new(GameState::new())
        .with_required_size(|_state: &mut GameState, size| {
            let _screen_w = size.x;
            let _screen_h = size.y;
            XY::new(MAP_W, MAP_H)
        })
        .with_draw(|state: &GameState, printer| {
            for y in 0..MAP_H {
                for x in 0..MAP_W {
                    let tile = state.tiles[y * MAP_W + x];
                    match tile {
                        Tile::Blank => printer.print(XY::new(x, y), "0"),
                        Tile::Wall => printer.print(XY::new(x, y), "1"),
                    }
                }
            }
        })
}
