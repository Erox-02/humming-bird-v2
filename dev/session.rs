use hbp100::{HBP100, SessionManager};

#[test]
fn test_session_persistent_counters() {
    let mut engine = HBP100::new();
    let session_mgr = SessionManager::new();
    
    let session_id = session_mgr.create_session(Some("test_intent"));
    
    let result1 = session_mgr.process_with_session(
        &mut engine.pipeline,
        &session_id,
        "Patient John Doe",
    ).unwrap();
    assert_eq!(result1.masked_text, "Patient [NAME_1]");
    assert_eq!(result1.metadata.get("[NAME_1]").unwrap(), "John Doe");
    let result2 = session_mgr.process_with_session(
        &mut engine.pipeline,
        &session_id,
        "Patient Jane Smith",
    ).unwrap();
    assert_eq!(result2.masked_text, "Patient [NAME_2]");
    assert_eq!(result2.metadata.get("[NAME_2]").unwrap(), "Jane Smith");
    assert!(result2.metadata.get("[NAME_1]").is_none());
    let restored = session_mgr.restore_with_session(
        &mut engine.pipeline,
        &session_id,
        "[NAME_1] and [NAME_2]",
    ).unwrap();
    assert_eq!(restored, "John Doe and Jane Smith");
}

#[test]
fn test_session_multiple_entity_types() {
    let mut engine = HBP100::new();
    let session_mgr = SessionManager::new();
    
    let session_id = session_mgr.create_session(None);
    
    let result1 = session_mgr.process_with_session(
        &mut engine.pipeline,
        &session_id,
        "John Doe, email john@example.com, phone 1234567890",
    ).unwrap();
    assert!(result1.masked_text.contains("[NAME_1]"));
    assert!(result1.masked_text.contains("[EMAIL_1]"));
    assert!(result1.masked_text.contains("[PHONE_1]"));
    let result2 = session_mgr.process_with_session(
        &mut engine.pipeline,
        &session_id,
        "Jane Smith, email jane@example.com, phone 9876543210",
    ).unwrap();
    assert!(result2.masked_text.contains("[NAME_2]"));
    assert!(result2.masked_text.contains("[EMAIL_2]"));
    assert!(result2.masked_text.contains("[PHONE_2]"));
}

#[test]
fn test_session_restore_preserves_order() {
    let mut engine = HBP100::new();
    let session_mgr = SessionManager::new();
    let session_id = session_mgr.create_session(None);
    session_mgr.process_with_session(
        &mut engine.pipeline,
        &session_id,
        "Alice",
    ).unwrap();
    session_mgr.process_with_session(
        &mut engine.pipeline,
        &session_id,
        "Bob",
    ).unwrap();
    
    session_mgr.process_with_session(
        &mut engine.pipeline,
        &session_id,
        "Charlie",
    ).unwrap();
    
    let restored = session_mgr.restore_with_session(
        &mut engine.pipeline,
        &session_id,
        "[NAME_3] [NAME_1] [NAME_2]",
    ).unwrap();
    
    assert_eq!(restored, "Charlie Alice Bob");
}

#[test]
fn test_stateless_process_unchanged() {
    let mut engine = HBP100::new();
    let result1 = engine.process("Patient John Doe", None);
    assert_eq!(result1.masked_text, "Patient [NAME_1]");
    let result2 = engine.process("Patient Jane Smith", None);
    assert_eq!(result2.masked_text, "Patient [NAME_1]");
    assert_ne!(result1.metadata.get("[NAME_1]").unwrap(), result2.metadata.get("[NAME_1]").unwrap());
}

#[test]
fn test_session_auto_creation() {
    let mut engine = HBP100::new();
    let session_mgr = SessionManager::new();
    let session_id = "test_session_123";
    let session_arc = session_mgr.get_or_create_session(session_id, Some("test"));
    
    {
        let mut session = session_arc.write().unwrap();
        assert_eq!(session.intent, Some("test".to_string()));
    }
    
    let result = session_mgr.process_with_session(
        &mut engine.pipeline,
        session_id,
        "Patient Test",
    ).unwrap();
    
    assert_eq!(result.masked_text, "Patient [NAME_1]");
}

#[test]
fn test_session_removal() {
    let session_mgr = SessionManager::new();
    let session_id = session_mgr.create_session(None);
    assert!(session_mgr.get_session(&session_id).is_some());
    assert!(session_mgr.session_count() == 1);
    assert!(session_mgr.remove_session(&session_id));
    assert!(session_mgr.get_session(&session_id).is_none());
    assert!(session_mgr.session_count() == 0);
}

#[test]
fn test_session_listing() {
    let session_mgr = SessionManager::new();
    let id1 = session_mgr.create_session(None);
    let id2 = session_mgr.create_session(None);
    let id3 = session_mgr.create_session(None);
    let sessions = session_mgr.list_sessions();
    assert_eq!(sessions.len(), 3);
    assert!(sessions.contains(&id1));
    assert!(sessions.contains(&id2));
    assert!(sessions.contains(&id3));
}