use std::ffi::CString;

/// Error returned by spfresh operations.
#[derive(Debug)]
pub struct Error {
    pub msg: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}

/// A BKT (Balanced K-Means Tree) in-memory index for `f32` vectors.
///
/// The index owns an opaque pointer to the underlying C++ `SPTAG::VectorIndex`.
/// On drop the C++ destructor is called to release memory.
pub struct BktIndex {
    ptr: *mut spfresh_sys::spfresh_index_t,
    dim: i32,
}

// The C++ index pointer is not thread-safe, but it is safe to move the
// `BktIndex` value between threads because only one thread can own it at a time.
unsafe impl Send for BktIndex {}

impl BktIndex {
    /// Create a new BKT index for `f32` vectors of the given dimension.
    ///
    /// Returns an error if the C++ `CreateInstance` fails (e.g. unsupported
    /// algorithm or value type).
    pub fn new(dim: i32) -> Result<Self, Error> {
        let algo = CString::new("BKT").unwrap();
        let vt = CString::new("Float").unwrap();

        // SAFETY: `spfresh_create` copies the strings before returning.
        let ptr = unsafe {
            spfresh_sys::spfresh_create(algo.as_ptr(), vt.as_ptr(), dim)
        };
        if ptr.is_null() {
            return Err(Error {
                msg: "failed to create BKT index".into(),
            });
        }
        Ok(BktIndex { ptr, dim })
    }

    /// Dimension of vectors in this index.
    pub fn dim(&self) -> i32 {
        self.dim
    }

    /// Set a build-time parameter on the underlying C++ index.
    ///
    /// `section` is the SPTAG config section name (e.g. `"Index"`).
    pub fn set_build_param(
        &mut self,
        name: &str,
        value: &str,
        section: &str,
    ) -> Result<(), Error> {
        let name_c = CString::new(name).map_err(|_| Error {
            msg: "null byte in param name".into(),
        })?;
        let value_c = CString::new(value).map_err(|_| Error {
            msg: "null byte in param value".into(),
        })?;
        let section_c = CString::new(section).map_err(|_| Error {
            msg: "null byte in param section".into(),
        })?;

        // SAFETY: the C function only reads the strings.
        unsafe {
            spfresh_sys::spfresh_set_build_param(
                self.ptr,
                name_c.as_ptr(),
                value_c.as_ptr(),
                section_c.as_ptr(),
            );
        }
        Ok(())
    }

    /// Build the index from a flat slice of `f32` vectors.
    ///
    /// `data` must contain `num_vectors * dim` elements.
    pub fn build(&mut self, data: &[f32], normalized: bool) -> Result<(), Error> {
        let num_vectors = data.len() as i32 / self.dim;
        let bytes = unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * 4,
            )
        };

        let ret = unsafe {
            spfresh_sys::spfresh_build(self.ptr, bytes.as_ptr(), num_vectors, normalized as i32)
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error { msg: "build failed".into() })
        }
    }

    /// Search for the `k` nearest neighbors of `query`.
    ///
    /// `query` must have length `dim`.
    /// Returns a vector of `(id, distance)` pairs, sorted by distance ascending.
    pub fn search(&self, query: &[f32], k: i32) -> Result<Vec<(i32, f32)>, Error> {
        if query.len() as i32 != self.dim {
            return Err(Error {
                msg: format!("query length {} != dim {}", query.len(), self.dim),
            });
        }

        let mut ids = vec![0i32; k as usize];
        let mut dists = vec![0.0f32; k as usize];

        let qbytes = unsafe {
            std::slice::from_raw_parts(
                query.as_ptr() as *const u8,
                query.len() * 4,
            )
        };

        let ret = unsafe {
            spfresh_sys::spfresh_search(
                self.ptr,
                qbytes.as_ptr(),
                k,
                ids.as_mut_ptr(),
                dists.as_mut_ptr(),
            )
        };
        if ret != 0 {
            return Err(Error { msg: "search failed".into() });
        }

        Ok(ids.into_iter().zip(dists).collect())
    }
}

impl Drop for BktIndex {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: pointer is valid and will not be used after this.
            unsafe { spfresh_sys::spfresh_free(self.ptr); }
        }
    }
}
