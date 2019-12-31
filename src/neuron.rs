use nalgebra::DMatrix;

/// Neuronal Network
pub struct NeuroNet {
    matrizes: Vec<DMatrix<f32>>,
}

impl NeuroNet {
    /// Generates a random neural Network
    pub fn random(tensors: &[usize]) -> NeuroNet {
        let matrizes = tensors
            .iter()
            .zip(tensors[1..].iter())
            .map(|(input, out)| DMatrix::new_random(*out, *input))
            .collect::<Vec<_>>();

        NeuroNet { matrizes }
    }
}
