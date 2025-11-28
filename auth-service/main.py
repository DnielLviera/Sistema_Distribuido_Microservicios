from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import jwt
import datetime
import hashlib
import os
import json
#import secrets
from dotenv import load_dotenv

load_dotenv()

app = FastAPI()

SECRET_KEY = os.getenv("SECRET_KEY")
ALGORITHM = os.getenv("ALGORITHM")
JSON_FILE = os.getenv("JSON_FILE")

class User(BaseModel):
    username: str
    password: str

class Token(BaseModel):
    access_token: str
    token_type: str

def load_users():
    if os.path.exists(JSON_FILE):
        with open(JSON_FILE, 'r') as file:
            return json.load(file)
    
    return []

def save_users(users):
    with open(JSON_FILE, 'w') as file:
        json.dump(users, file, indent=2)

def hash_password(password: str) -> str:
    return hashlib.sha256(password.encode()).hexdigest()

def create_acces_token(username: str):
    expire = datetime.datetime.utcnow() + datetime.timedelta(hours=24)
    to_encode = {"sub": username, "exp": expire}
    encoded_jwt = jwt.encode(to_encode, SECRET_KEY, ALGORITHM)
    return encoded_jwt

@app.get("/")
def read_root():
    return {"message": "Auth Service funcionando"}

@app.post("/register")
def register(user: User):
    try:
        users_db = load_users()

        for u in users_db:
            if u["username"] == user.username:
                raise HTTPException(status_code=400, detail="Usuario ya existe")

        hashed_password = hash_password(user.password)
        new_user = {"username": user.username, "password": hashed_password}
        users_db.append(new_user)
        save_users(users_db)

        return {"message": "Usuario registrado exitosamente"}
    
    except Exception as e:
        print(f"Error en register: {e}")
        raise HTTPException(status_code=500, detail="Error interno del servidor")

@app.post("/login", response_model=Token)
def login(user: User):
    try:
        users_db = load_users()
        input_hashed_password = hash_password(user.password)

        for u in users_db:
            if u["username"] == user.username and u["password"] == input_hashed_password:
                access_token = create_acces_token(user.username)
                return {"access_token": access_token, "token_type": "bearer"}

        raise HTTPException(status_code=401, detail="Credenciales incorrectas")
    
    except Exception as e:
        print(f"Error en login: {e}")
        raise HTTPException(status_code=500, detail="Error interno del servidor")

@app.get("/users")
def get_users():
    users_db = load_users()
    return {"users": users_db}