use pyo3::prelude::*;

const MASS: f32 = 0.1;
const STIFFNESS: f32 = 10.0;


#[pyfunction]
fn string(initial_condition: Vec<f32>, iterations: usize, step_size: f32, mass: f32, stiffness: f32) -> Vec<Vec<f32>> {
    let mut solution: Vec<Vec<f32>> = Vec::new();
    solution.push(initial_condition);
    for i in 0..iterations-1 {
        solution.push(string_ode(&solution[i], step_size, mass, stiffness))
    }
    solution
}


fn string_ode(cond: &Vec<f32>, step_size: f32, mass: f32, stiffness: f32) -> Vec<f32> {
    let mut next: Vec<f32> = Vec::new();
    let half = cond.len()/2;
    let full = cond.len();
    let c = step_size * stiffness / mass;
    // push first half POSITION
    for i in 0..half {
        next.push(cond[i] + step_size * cond[half+i]);
    }
    // VELOCITY
    // push left end
    next.push(cond[half] + c * (cond[1] - 2.0 * cond[0]));
    // push second half
    for i in 1..half-1 {
        next.push(cond[half+i] + c * (cond[i-1] + cond[i+1] - 2.0 * cond[i]));
    }
    // push right half
    next.push(cond[full-1] + c * (cond[half-2] - 2.0 * cond[half-1]));
    next
}


// #[pyfunction]
// fn simple_string(initial_condition: [f32; 6], iterations: usize, step_size: f32) -> Vec<[f32; 6]> {
//     let mut solution: Vec<[f32; 6]> = Vec::new();
//     solution.push(initial_condition);
//     for i in 0..iterations-1 {
//         solution.push(simple_string_ode(solution[i], step_size));
//     }
//     solution
// }


// fn simple_string_ode(cond: [f32; 6], step_size: f32) -> [f32; 6] {
//     [
//         cond[0] + step_size * cond[3],
//         cond[1] + step_size * cond[4],
//         cond[2] + step_size * cond[5],
//         cond[3] + step_size * STIFFNESS/MASS * (cond[1] - 2.0 * cond[0]),
//         cond[4] + step_size * STIFFNESS/MASS * (cond[0] + cond[2] - 2.0 * cond[1]),
//         cond[5] + step_size * STIFFNESS/MASS * (cond[1] - 2.0 * cond[2]),
//     ]
// }


#[pyfunction]
fn coupled_pendulum(initial_condition: [f32; 4], step_size: f32, iterations: u32) -> Vec<[f32; 4]> {
    let mut solution: Vec<[f32; 4]> = Vec::new();
    solution.push(initial_condition);
    for _ in 0..iterations {
        solution.push(coupled_pendulum_ode(solution[solution.len()-1], step_size));
    }
    solution
}

fn coupled_pendulum_ode(cond: [f32; 4], step_size: f32) -> [f32; 4] {
    [
        cond[0] + step_size * cond[1],
        cond[1] + step_size * STIFFNESS/MASS*(cond[2]-2.0*cond[0]),
        cond[2] + step_size * cond[3],
        cond[3] + step_size * -STIFFNESS/MASS*(cond[2]-cond[0]-1.0)
    ]
}

/// A Python module implemented in Rust.
#[pymodule]
fn simulations(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(coupled_pendulum, m)?)?;
    m.add_function(wrap_pyfunction!(simple_string, m)?)?;
    m.add_function(wrap_pyfunction!(string, m)?)?;
    Ok(())
}
