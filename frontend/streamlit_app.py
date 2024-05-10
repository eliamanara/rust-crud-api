# streamlit_app.py

import hmac
import pandas as pd
import streamlit as st
from streamlit_extras.switch_page_button import switch_page

from auth import check_password


def main_page():
    # Initialize connection.
    conn = st.connection("postgresql", type="sql")

    if not check_password():
        st.stop()

    # Perform query.
    df = conn.query('SELECT id, author, title FROM books;', ttl="10m")

    st.dataframe(df, hide_index=True, use_container_width=True)

    # # Print results.
    # for row in df.itertuples():
    #     st.write(f"{row.author} wrote {row.title}"


def book():
    st.markdown("# Page 2 ❄️")
    st.sidebar.markdown("# Page 2 ❄️")


def author():
    st.markdown("# Page 3 🎉")
    st.sidebar.markdown("# Page 3 🎉")


page_names_to_funcs = {
    "Main Page": main_page,
    "Page 2": book,
    "Page 3": author,
}
selected_page = st.sidebar.selectbox(
    "Select a page", page_names_to_funcs.keys())
page_names_to_funcs[selected_page]()
