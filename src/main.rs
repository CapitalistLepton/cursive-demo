use cursive::event::{Event, EventResult};
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
    player_location: XY<usize>,
}

impl GameState {
    fn new() -> Self {
        let mut state = Self {
            tiles: [Tile::Blank; MAP_W * MAP_H],
            player_location: XY::new(MAP_W / 2, MAP_H / 2),
        };
        // Setup walls around the perimeter
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
            // Draw tiles first
            for y in 0..MAP_H {
                for x in 0..MAP_W {
                    let tile = state.tiles[y * MAP_W + x];
                    match tile {
                        Tile::Blank => printer.print(XY::new(x, y), "0"),
                        Tile::Wall => printer.print(XY::new(x, y), "1"),
                    }
                }
            }
            // Draw player on top
            printer.print(state.player_location, "@");
        })
        .with_on_event(|state: &mut GameState, event| match event {
            Event::WindowResize => todo!(),
            Event::Char(k) => {
                if k == 'w' {
                    state.player_location.y -= 1;
                } else if k == 's' {
                    state.player_location.y += 1;
                } else if k == 'a' {
                    state.player_location.x -= 1;
                } else if k == 'd' {
                    state.player_location.x += 1;
                }
                EventResult::Ignored
            }
            _ => todo!(),
        })
}
