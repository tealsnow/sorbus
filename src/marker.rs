use {
    crate::{Event, ParserTrait, TokenKindTrait},
    drop_bomb::DebugDropBomb,
    std::error::Error,
};

#[derive(Debug)]
pub struct Marker {
    pos: usize,
    bomb: DebugDropBomb,
}

impl Marker {
    pub fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DebugDropBomb::new("Markers need to be completed"),
        }
    }

    pub fn complete<K: TokenKindTrait, E: Error + Clone + PartialEq, P: ParserTrait<K, E>>(
        mut self,
        p: &mut P,
        kind: K,
    ) -> CompletedMarker {
        self.bomb.defuse();

        let event_at_pos = &mut p.events()[self.pos];
        debug_assert_eq!(*event_at_pos, Event::Placeholder);

        *event_at_pos = Event::StartNode {
            kind,
            forward_parent: None,
        };

        p.events().push(Event::FinishNode);

        CompletedMarker { pos: self.pos }
    }
}

#[derive(Debug)]
pub struct CompletedMarker {
    pos: usize,
}

impl CompletedMarker {
    pub fn precede<K: TokenKindTrait, E: Error + Clone + PartialEq, P: ParserTrait<K, E>>(
        self,
        p: &mut P,
    ) -> Marker {
        let new_m = p.node_start();

        if let Event::StartNode {
            ref mut forward_parent,
            ..
        } = p.events()[self.pos]
        {
            *forward_parent = Some(new_m.pos - self.pos)
        } else {
            unreachable!()
        }

        new_m
    }
}
