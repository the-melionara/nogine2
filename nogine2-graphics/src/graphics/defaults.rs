use std::sync::{Arc, RwLock};

use nogine2_core::crash;

use super::shader::{Shader, SubShader, SubShaderType};

const BATCH_VERT_SRC: &[u8] = include_bytes!("../../shaders/batch.vert");
const BATCH_FRAG_SRC: &[u8] = include_bytes!("../../shaders/batch.frag");


static DEF_SUBSHADERS: RwLock<Option<DefaultSubShaders>> = RwLock::new(None);
static DEF_SHADERS: RwLock<Option<DefaultShaders>> = RwLock::new(None);

/// Singleton containing all the default subshaders.
pub struct DefaultSubShaders {
    batch_vert: Arc<SubShader>,
    batch_frag: Arc<SubShader>,
}

impl DefaultSubShaders {
    pub(crate) fn init() -> bool {
        let Some(batch_vert) = SubShader::new(BATCH_VERT_SRC, SubShaderType::Vertex) else { return false };
        let Some(batch_frag) = SubShader::new(BATCH_FRAG_SRC, SubShaderType::Fragment) else { return false };

        let Ok(mut subshaders) = DEF_SUBSHADERS.write() else { crash!("Couldn't access DefaultSubShaders singleton!") };
        *subshaders = Some(DefaultSubShaders { batch_vert, batch_frag });
        return true;
    }

    /// Default vertex subshader for rendering batch meshes.
    pub fn batch_vert() -> Arc<SubShader> {
        let Ok(subshaders) = DEF_SUBSHADERS.read() else { crash!("Couldn't access DefaultSubShaders singleton!") };
        let Some(subshaders) = subshaders.as_ref() else { crash!("DefaultSubShaders is not initialized!") };
        subshaders.batch_vert.clone()
    }

    /// Default fragment subshader for rendering batch meshes.
    pub fn batch_frag() -> Arc<SubShader> {
        let Ok(subshaders) = DEF_SUBSHADERS.read() else { crash!("Couldn't access DefaultSubShaders singleton!") };
        let Some(subshaders) = subshaders.as_ref() else { crash!("DefaultSubShaders is not initialized!") };
        subshaders.batch_frag.clone()
    }
}


/// Singleton containing all the default shaders.
pub struct DefaultShaders {
    batch: Arc<Shader>,
}

impl DefaultShaders {
    pub(crate) fn init() -> bool {
        let Ok(subshaders) = DEF_SUBSHADERS.read() else { crash!("Couldn't access DefaultSubShaders singleton!") };
        let Some(subshaders) = subshaders.as_ref() else { crash!("DefaultSubShaders is not initialized!") };

        let Some(batch) = Shader::new(&subshaders.batch_vert, &subshaders.batch_frag) else { return false };

        let Ok(mut shaders) = DEF_SHADERS.write() else { crash!("Couldn't access DefaultShaders singleton!") };
        *shaders = Some(DefaultShaders { batch });
        return true;
    }

    /// Default shader for rendering batch meshes.
    pub fn batch() -> Arc<Shader> {
        let Ok(shaders) = DEF_SHADERS.read() else { crash!("Couldn't access DefaultShaders singleton!") };
        let Some(shaders) = shaders.as_ref() else { crash!("DefaultShaders is not initialized!") };
        shaders.batch.clone()
    }
}
