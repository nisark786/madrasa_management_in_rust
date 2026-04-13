import requests
import json

# Get test token (assuming a test user exists)
BASE_URL = "http://localhost:8000/api/v1"

# Test 1: Check available fields
print("Test 1: GET /reports/available-fields")
try:
    response = requests.get(f"{BASE_URL}/reports/available-fields")
    print(f"Status: {response.status_code}")
    if response.status_code == 200:
        fields = response.json()
        print(f"✓ Available fields retrieved: {len(fields)} fields")
        print(f"  Sample fields: {fields[:3]}")
    else:
        print(f"✗ Error: {response.text}")
except Exception as e:
    print(f"✗ Exception: {e}")

print("\n" + "="*60 + "\n")

# Test 2: Test quick export (CSV)
print("Test 2: POST /reports/export/quick (CSV)")
try:
    payload = {
        "fields": ["first_name", "last_name", "email", "class_name"],
        "format": "csv",
        "filters": {}
    }
    response = requests.post(
        f"{BASE_URL}/reports/export/quick",
        json=payload,
        headers={"Authorization": "Bearer test"}  # This will likely fail without valid token
    )
    print(f"Status: {response.status_code}")
    if response.status_code in [200, 401]:
        print(f"Response: {response.text[:200]}")
    else:
        print(f"Error: {response.text}")
except Exception as e:
    print(f"Exception: {e}")

print("\n" + "="*60 + "\n")

# Test 3: Test quick export (Excel)
print("Test 3: POST /reports/export/quick (Excel)")
try:
    payload = {
        "fields": ["first_name", "last_name", "email"],
        "format": "xlsx",
        "filters": {}
    }
    response = requests.post(
        f"{BASE_URL}/reports/export/quick",
        json=payload,
        headers={"Authorization": "Bearer test"}
    )
    print(f"Status: {response.status_code}")
    print(f"Content-Type: {response.headers.get('content-type', 'N/A')}")
except Exception as e:
    print(f"Exception: {e}")

print("\n" + "="*60 + "\n")

# Test 4: Test quick export (PDF)
print("Test 4: POST /reports/export/quick (PDF)")
try:
    payload = {
        "fields": ["first_name", "last_name", "email"],
        "format": "pdf",
        "filters": {}
    }
    response = requests.post(
        f"{BASE_URL}/reports/export/quick",
        json=payload,
        headers={"Authorization": "Bearer test"}
    )
    print(f"Status: {response.status_code}")
    print(f"Content-Type: {response.headers.get('content-type', 'N/A')}")
except Exception as e:
    print(f"Exception: {e}")

print("\nTest suite completed!")
