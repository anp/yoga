use ffi_types::value::Value;

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Edge {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
    Start = 4,
    End = 5,
    Horizontal = 6,
    Vertical = 7,
    All = 8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Edges([Option<Value>; 9]);

impl Edges {
    pub fn empty() -> Self {
        Edges([None, None, None, None, None, None, None, None, None])
    }

    pub fn computed(&mut self, edge: Edge) -> Option<Value> {
        if let Some(v) = self[edge] {
            return Some(v);
        }

        use Edge::*;
        match edge {
            Top | Bottom => if let Some(v) = self[Edge::Vertical] {
                return Some(v);
            },

            Left | Right | Start | End => if let Some(v) = self[Edge::Horizontal] {
                return Some(v);
            },
        };

        if let Some(v) = self[Edge::All] {
            return Some(v);
        }

        return None;
    }
}

impl ::std::ops::Index<Edge> for Edges {
    type Output = Option<Value>;
    fn index(&self, index: Edge) -> &Self::Output {
        // UNSAFE(anp): we know that Edge and Edges cannot index incorrectly
        unsafe { &self.0.get_unchecked(index as usize) }
    }
}

impl ::std::ops::IndexMut<Edge> for Edges {
    fn index_mut(&mut self, index: Edge) -> &mut Self::Output {
        // UNSAFE(anp): we know that Edge and Edges cannot index incorrectly
        unsafe { &mut self.0.get_unchecked_mut(index as usize) }
    }
}
