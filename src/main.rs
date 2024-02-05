use cursive::event::{Event, EventResult};
use cursive::views::Canvas;
use cursive::{Cursive, CursiveExt, XY};

const MAP_W: usize = 15;
const MAP_H: usize = 10;

#[derive(Copy, Clone, PartialEq)]
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
    fn in_bounds(&self, location: XY<usize>) -> bool {
        location.x < MAP_W && location.y < MAP_H
    }
    fn in_bounds_and_blank(&self, location: XY<usize>) -> bool {
        self.in_bounds(location) && self.tiles[location.y * MAP_W + location.x] == Tile::Blank
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
                    let mut loc = state.player_location.clone();
                    loc.y -= 1;
                    if state.in_bounds_and_blank(loc) {
                        state.player_location = loc;
                    }
                } else if k == 's' {
                    let mut loc = state.player_location.clone();
                    loc.y += 1;
                    if state.in_bounds_and_blank(loc) {
                        state.player_location = loc;
                    }
                } else if k == 'a' {
                    let mut loc = state.player_location.clone();
                    loc.x -= 1;
                    if state.in_bounds_and_blank(loc) {
                        state.player_location = loc;
                    }
                } else if k == 'd' {
                    let mut loc = state.player_location.clone();
                    loc.x += 1;
                    if state.in_bounds_and_blank(loc) {
                        state.player_location = loc;
                    }
                }
                EventResult::Ignored
            }
            _ => todo!(),
        })
}
