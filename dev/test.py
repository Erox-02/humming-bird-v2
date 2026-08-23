from hbp100 import HBP100

def main():
        
    engine = HBP100()
    
    test_texts = [
        "Patient John Doe, MRN: 123456, Phone: 9876543210",
        "The patient was diagnosed with diabetes",
        "Contact: john.doe@email.com, Address: 123 Main St",
    ]
    
    for text in test_texts:
        print(f"\n Input: {text}")
        result = engine.process(text)
        print(f"Masked: {result['masked_text']}")
        print(f"Entities: {len(result['entities'])}")
        print(f"Has PII: {result['has_pii']}")
        
        for decision in result['decisions'][:3]:
            print(f"  {decision['entity_type']} -> {decision['decision']} ({decision['confidence']})")

if __name__ == "__main__":
    main()