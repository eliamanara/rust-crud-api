import hmac
import pandas as pd
import streamlit as st

# Initialize connection.
conn = st.connection("postgresql", type="sql")


def check_password():
    """Returns `True` if the user had a correct password."""

    def login_form():
        """Form with widgets to collect user information"""
        with st.form("Credentials"):
            st.text_input("Username", key="username")
            st.text_input("Password", type="password", key="password")
            st.form_submit_button("Log in", on_click=password_entered)

    def password_entered():
        """Checks whether a password entered by the user is correct."""
        if check_auth():
            st.session_state["password_correct"] = True
            # Don't store the username or password.
            del st.session_state["password"]
            del st.session_state["username"]
        else:
            st.session_state["password_correct"] = False

    def check_auth():
        login_check = False
        try:
            username = st.session_state["username"]
            # print("Entered username: ", username)
            password = st.session_state["password"]
            # print("Entered password: ", password)

            query = 'SELECT password FROM users where username = \'' + username + '\';'
            # print(query)
            db_password = conn.query(
                query, ttl="10m"
            )
            # print("Extracted password:", db_password)
            if db_password.values[0] == password:
                login_check = True
            del db_password
            del username
            del password
        except:
            del db_password
            del username
            del password
            return False
        return login_check

    # Return True if the username + password is validated.
    if st.session_state.get("password_correct", False):
        return True

    # Show inputs for username + password.
    login_form()
    if "password_correct" in st.session_state:
        st.error("😕 User not known or password incorrect")
    return False
