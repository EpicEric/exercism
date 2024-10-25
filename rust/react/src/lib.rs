use std::{
    cell::{self, RefCell},
    collections::{hash_map::Entry, HashMap},
    sync::atomic::{AtomicUsize, Ordering},
};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: T,
    reactors: Vec<CellId>,
}

struct ComputeCell<'a, T> {
    computation: Box<dyn Fn(&[T]) -> T + 'a>,
    dependencies: Vec<CellId>,
    reactors: Vec<CellId>,
    callbacks: HashMap<CallbackId, Vec<RefCell<(Box<dyn FnMut(T) + 'a>, Option<T>)>>>,
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    input_cells: HashMap<CellId, InputCell<T>>,
    compute_cells: HashMap<CellId, ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: HashMap::new(),
            compute_cells: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = InputCellId(COUNTER.fetch_add(1, Ordering::AcqRel));
        self.input_cells.insert(
            CellId::Input(id),
            InputCell {
                value: initial,
                reactors: vec![],
            },
        );
        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let id = ComputeCellId(COUNTER.fetch_add(1, Ordering::AcqRel));
        for dependency in dependencies {
            if !self.input_cells.contains_key(dependency)
                && !self.compute_cells.contains_key(dependency)
            {
                return Err(*dependency);
            }
        }
        let mut dependencies_vec: Vec<_> = dependencies.iter().copied().collect();
        while let Some(dependency) = dependencies_vec.pop() {
            if let Some(cell) = self.input_cells.get_mut(&dependency) {
                cell.reactors.push(CellId::Compute(id));
            } else if let Some(cell) = self.compute_cells.get_mut(&dependency) {
                cell.reactors.push(CellId::Compute(id));
                // Recurse through the reactive chain
                for transitive_dependency in cell.dependencies.clone() {
                    dependencies_vec.push(transitive_dependency.clone());
                }
            } else {
                unreachable!()
            }
        }
        self.compute_cells.insert(
            CellId::Compute(id),
            ComputeCell {
                computation: Box::new(compute_func),
                dependencies: dependencies.iter().copied().collect(),
                reactors: vec![],
                callbacks: HashMap::new(),
            },
        );
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match self
            .input_cells
            .get(&id)
            .map(|InputCell { value, .. }| value)
            .copied()
        {
            Some(x) => Some(x),
            None => self.compute_cells.get(&id).map(|cell| {
                let dependencies: Vec<T> = cell
                    .dependencies
                    .iter()
                    .map(|&cell_id| self.value(cell_id).unwrap())
                    .collect();
                let value = (cell.computation)(dependencies.as_slice());
                value
            }),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        let reactors = match self
            .input_cells
            .entry(CellId::Input(id))
            .and_modify(|cell| cell.value = new_value)
        {
            Entry::Occupied(entry) => entry.get().reactors.clone(),
            Entry::Vacant(_) => return false,
        };
        reactors.iter().for_each(|cell_id| {
            let compute_cell = self
                .compute_cells
                .get(cell_id)
                .expect("Compute cell was not removed");
            let dependencies: Vec<T> = compute_cell
                .dependencies
                .iter()
                .map(|&dep_id| self.value(dep_id).unwrap())
                .collect();
            let value = (compute_cell.computation)(dependencies.as_slice());
            let compute_cell = self
                .compute_cells
                .get(cell_id)
                .expect("Compute cell was not removed");
            dbg!(cell_id);
            for cb_vec in compute_cell.callbacks.values() {
                cb_vec.iter().for_each(|cb| {
                    let mut cb = cb.borrow_mut();
                    if let Some(previous_value) = cb.1 {
                        if previous_value != value {
                            (cb.0)(value)
                        }
                        cb.1 = Some(value);
                    }
                });
            }
        });
        true
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let value = self.value(CellId::Compute(id));
        if let Some(cell) = self.compute_cells.get_mut(&CellId::Compute(id)) {
            let callback_id = CallbackId(COUNTER.fetch_add(1, Ordering::AcqRel));
            let callback_entry = cell.callbacks.entry(callback_id).or_default();
            callback_entry.push(RefCell::new((Box::new(callback), value)));
            Some(callback_id)
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(cell) = self.compute_cells.get_mut(&CellId::Compute(cell)) {
            match cell.callbacks.entry(callback) {
                Entry::Occupied(occupied_entry) => {
                    occupied_entry.remove_entry();
                    Ok(())
                }
                Entry::Vacant(_) => Err(RemoveCallbackError::NonexistentCallback),
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
