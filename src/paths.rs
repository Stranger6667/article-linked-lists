/// A node in a linked list representing a JSON pointer.
#[derive(Debug)]
pub(crate) struct JsonPointerNode<'a, 'b> {
    pub(crate) segment: Option<&'a str>,
    pub(crate) parent: Option<&'b JsonPointerNode<'a, 'b>>,
}
impl<'a, 'b> JsonPointerNode<'a, 'b> {
    /// Create a root node of a JSON pointer.
    pub(crate) const fn new() -> Self {
        JsonPointerNode {
            segment: None,
            parent: None,
        }
    }
    /// Push a new segment to the JSON pointer.
    pub(crate) fn push(&'a self, segment: &'a str) -> JsonPointerNode<'a, 'b> {
        JsonPointerNode {
            segment: Some(segment),
            parent: Some(self),
        }
    }
    /// Convert the JSON pointer node to a vector of path segments.
    pub(crate) fn to_vec(&self) -> Vec<String> {
        // Walk the linked list to calculate the capacity
        let mut capacity = 0;
        let mut head = self;
        while let Some(next) = head.parent {
            head = next;
            capacity += 1;
        }
        // Callect the segments from the head to the tail
        let mut buffer = Vec::with_capacity(capacity);
        let mut head = self;
        if let Some(segment) = &head.segment {
            buffer.push((*segment).to_string());
        }
        while let Some(next) = head.parent {
            head = next;
            if let Some(segment) = &head.segment {
                buffer.push((*segment).to_string());
            }
        }
        // Reverse the buffer to get the segments in the correct order
        buffer.reverse();
        buffer
    }
}
