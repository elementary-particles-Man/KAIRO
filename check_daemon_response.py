import requests
r = requests.post('http://127.0.0.1:8080/assign_p_address', json={'public_key': 'test_key'})
print(r.text)