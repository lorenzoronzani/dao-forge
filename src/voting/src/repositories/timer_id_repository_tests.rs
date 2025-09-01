use super::*;
use ic_cdk_timers::TimerId;

fn create_test_timer_id() -> TimerId {
    unsafe { std::mem::transmute(42u64) }
}

#[test]
fn test_timer_id_repository_save_and_get() {
    let timer_id = 123u32;
    let ic_timer_id = create_test_timer_id();

    TimerIdRepository::save(timer_id, ic_timer_id);

    let retrieved_timer_id = TimerIdRepository::get(timer_id);

    assert!(retrieved_timer_id.is_some());
    assert_eq!(retrieved_timer_id.unwrap(), ic_timer_id);
}

#[test]
fn test_timer_id_repository_get_nonexistent() {
    let result = TimerIdRepository::get(99999);
    assert!(result.is_none());
}

#[test]
fn test_timer_id_repository_save_multiple() {
    let timer_id1 = 200u32;
    let timer_id2 = 201u32;
    let ic_timer_id1 = unsafe { std::mem::transmute(100u64) };
    let ic_timer_id2 = unsafe { std::mem::transmute(200u64) };

    TimerIdRepository::save(timer_id1, ic_timer_id1);
    TimerIdRepository::save(timer_id2, ic_timer_id2);

    let retrieved1 = TimerIdRepository::get(timer_id1).unwrap();
    let retrieved2 = TimerIdRepository::get(timer_id2).unwrap();

    assert_eq!(retrieved1, ic_timer_id1);
    assert_eq!(retrieved2, ic_timer_id2);
    assert_ne!(retrieved1, retrieved2);
}

#[test]
fn test_timer_id_repository_delete() {
    let timer_id = 300u32;
    let ic_timer_id = unsafe { std::mem::transmute(300u64) };

    TimerIdRepository::save(timer_id, ic_timer_id);

    assert!(TimerIdRepository::get(timer_id).is_some());

    TimerIdRepository::delete(timer_id);

    assert!(TimerIdRepository::get(timer_id).is_none());
}

#[test]
fn test_timer_id_repository_delete_nonexistent() {
    TimerIdRepository::delete(88888);

    assert!(TimerIdRepository::get(88888).is_none());
}

#[test]
fn test_timer_id_repository_overwrite() {
    let timer_id = 400u32;
    let ic_timer_id1 = unsafe { std::mem::transmute(400u64) };
    let ic_timer_id2 = unsafe { std::mem::transmute(500u64) };

    TimerIdRepository::save(timer_id, ic_timer_id1);
    assert_eq!(TimerIdRepository::get(timer_id).unwrap(), ic_timer_id1);

    TimerIdRepository::save(timer_id, ic_timer_id2);

    let retrieved = TimerIdRepository::get(timer_id).unwrap();
    assert_eq!(retrieved, ic_timer_id2);
    assert_ne!(retrieved, ic_timer_id1);
}

#[test]
fn test_timer_id_repository_workflow() {
    let timer_id = 500u32;
    let ic_timer_id = unsafe { std::mem::transmute(500u64) };

    assert!(TimerIdRepository::get(timer_id).is_none());

    TimerIdRepository::save(timer_id, ic_timer_id);

    let retrieved = TimerIdRepository::get(timer_id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), ic_timer_id);

    TimerIdRepository::delete(timer_id);

    assert!(TimerIdRepository::get(timer_id).is_none());
}
