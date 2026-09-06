use std::ffi::CString;

#[test]
fn bkt_build_and_search() {
    let algo = CString::new("BKT").unwrap();
    let vt = CString::new("Float").unwrap();

    let idx = unsafe {
        spfresh_sys::spfresh_create(algo.as_ptr(), vt.as_ptr(), 10)
    };
    assert!(!idx.is_null(), "spfresh_create returned null");

    let name = CString::new("DistCalcMethod").unwrap();
    let value = CString::new("L2").unwrap();
    let section = CString::new("Index").unwrap();
    unsafe {
        spfresh_sys::spfresh_set_build_param(
            idx,
            name.as_ptr(),
            value.as_ptr(),
            section.as_ptr(),
        );
    }

    // 100 vectors of dim 10
    let data: Vec<f32> = (0..1000).map(|i| i as f32).collect();
    let bytes = unsafe {
        std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 4)
    };

    let ret = unsafe {
        spfresh_sys::spfresh_build(idx, bytes.as_ptr(), 100, 0)
    };
    assert_eq!(ret, 0, "spfresh_build failed");

    // Use the first vector as query (expected nearest neighbor: id 0, dist 0)
    let query = &data[..10];
    let qbytes = unsafe {
        std::slice::from_raw_parts(query.as_ptr() as *const u8, query.len() * 4)
    };

    let mut ids = vec![0i32; 5];
    let mut dists = vec![0.0f32; 5];

    let ret = unsafe {
        spfresh_sys::spfresh_search(
            idx,
            qbytes.as_ptr(),
            5,
            ids.as_mut_ptr(),
            dists.as_mut_ptr(),
        )
    };
    assert_eq!(ret, 0, "spfresh_search failed");
    assert_eq!(ids[0], 0, "nearest neighbor should be id 0");
    assert_eq!(dists[0], 0.0, "distance to identical vector should be 0");

    unsafe {
        spfresh_sys::spfresh_free(idx);
    }
}
