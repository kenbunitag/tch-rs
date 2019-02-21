use crate::tensor::Tensor;

pub struct BatchNorm2DConfig {
    cudnn_enabled: bool,
    eps: f64,
    momentum: f64,
}

impl Default for BatchNorm2DConfig {
    fn default() -> Self {
        BatchNorm2DConfig {
            cudnn_enabled: true,
            eps: 1e-5,
            momentum: 0.1,
        }
    }
}

pub struct BatchNorm2D {
    config: BatchNorm2DConfig,
    running_mean: Tensor,
    running_var: Tensor,
    ws: Tensor,
    bs: Tensor,
}

impl BatchNorm2D {
    pub fn new(
        vs: &mut super::var_store::VarStore,
        out_dim: i64,
        config: BatchNorm2DConfig,
    ) -> BatchNorm2D {
        BatchNorm2D {
            config,
            running_mean: vs.zeros(&[out_dim]),
            running_var: vs.ones(&[out_dim]),
            ws: vs.uniform(&[out_dim], 0.0, 1.0),
            bs: vs.zeros(&[out_dim]),
        }
    }
}

impl super::module::ModuleT for BatchNorm2D {
    fn forward_t(&self, xs: &Tensor, train: bool) -> Tensor {
        Tensor::batch_norm(
            xs,
            Some(&self.ws),
            Some(&self.bs),
            Some(&self.running_mean),
            Some(&self.running_var),
            train,
            self.config.momentum,
            self.config.eps,
            self.config.cudnn_enabled,
        )
    }
}
