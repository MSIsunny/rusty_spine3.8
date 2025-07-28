use crate::{
    c::{
        sp38SkeletonClipping_clipEnd, sp38SkeletonClipping_clipEnd2,
        sp38SkeletonClipping_clipStart, sp38SkeletonClipping_clipTriangles,
        sp38SkeletonClipping_create, sp38SkeletonClipping_dispose, sp38SkeletonClipping_isClipping,
        spSkeletonClipping,
    },
    c_interface::SyncPtr,
    clipping_attachment::ClippingAttachment,
    slot::Slot,
};

#[cfg(doc)]
use crate::draw::SimpleDrawer;

/// Active state for [`ClippingAttachment`] during mesh generation.
///
/// For example usage, see the [`SimpleDrawer::draw`] implementation.
#[derive(Debug)]
pub struct SkeletonClipping {
    c_skeleton_clipping: SyncPtr<spSkeletonClipping>,
    owns_memory: bool,
}

impl Default for SkeletonClipping {
    fn default() -> Self {
        Self::new()
    }
}

impl SkeletonClipping {
    #[must_use]
    pub fn new() -> Self {
        Self {
            c_skeleton_clipping: unsafe { SyncPtr(sp38SkeletonClipping_create()) },
            owns_memory: true,
        }
    }

    pub fn clip_start(&mut self, slot: &Slot, clip: &ClippingAttachment) {
        unsafe {
            sp38SkeletonClipping_clipStart(self.c_ptr_mut(), slot.c_ptr(), clip.c_ptr());
        }
    }

    pub fn clip_end(&mut self, slot: &Slot) {
        unsafe {
            sp38SkeletonClipping_clipEnd(self.c_ptr_mut(), slot.c_ptr());
        }
    }

    pub fn clip_end2(&mut self) {
        unsafe {
            sp38SkeletonClipping_clipEnd2(self.c_ptr_mut());
        }
    }

    #[must_use]
    pub fn is_clipping(&self) -> bool {
        unsafe { sp38SkeletonClipping_isClipping(self.c_ptr_mut()) != 0 }
    }

    /// # Safety
    ///
    /// The triangles data passed in must represent valid mesh data.
    pub unsafe fn clip_triangles(
        &self,
        vertices: &mut [[f32; 2]],
        triangles: &mut [u16],
        uvs: &mut [[f32; 2]],
        stride: i32,
    ) {
        sp38SkeletonClipping_clipTriangles(
            self.c_ptr(),
            vertices.as_mut_ptr().cast::<f32>(),
            vertices.len() as i32,
            triangles.as_mut_ptr(),
            triangles.len() as i32,
            uvs.as_mut_ptr().cast::<f32>(),
            stride,
        );
    }

    c_ptr!(c_skeleton_clipping, spSkeletonClipping);
    // TODO
    /*spTriangulator *triangulator;
    spFloatArray *clippingPolygon;
    spFloatArray *clipOutput;
    spFloatArray *clippedVertices;
    spFloatArray *clippedUVs;
    spUnsignedShortArray *clippedTriangles;
    spFloatArray *scratch;
    spClippingAttachment *clipAttachment;
    spArrayFloatArray *clippingPolygons;*/
}

impl Drop for SkeletonClipping {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                sp38SkeletonClipping_dispose(self.c_skeleton_clipping.0);
            }
        }
    }
}
