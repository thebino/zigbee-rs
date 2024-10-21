use basemgt::{AIBAttribute, AIBAttributeValue, ApsmeBindConfirm, ApsmeBindRequest};

pub(crate) mod basemgt;
pub(crate) mod groupmgt;

// 2.2.4.2
// Application support sub-layer management service = service access point
//
// supports the transport of management commands between the NHLE and the APSME
pub(crate) trait ApsmeSap {
    // 2.2.4.3.1
    // request to bind two devices together, or to bind a device to a group
    fn bind_request(request: ApsmeBindRequest) -> ApsmeBindConfirm;
    // 2.2.4.3.3
    // request to unbind two devices, or to unbind a device from a group
    fn unbind_request();
    // 2.2.4.4.1
    fn get(attribute: AIBAttribute) -> AIBAttributeValue;
    // 2.2.4.4.3
    fn set();
    // 2.2.4.5.1
    fn add_group();
    // 2.2.4.5.1
    fn remove_group();
    // 2.2.4.5.5
    fn remove_all_groups();
}

// TODO: add AIB (APS information base) a database of managed objects
//
// 2.2.4.4.1
