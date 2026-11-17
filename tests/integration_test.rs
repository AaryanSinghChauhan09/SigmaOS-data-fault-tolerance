//! Integration tests for SigmaOS-data-fault-tolerance.

use sigmaos_data_fault_tolerance_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
