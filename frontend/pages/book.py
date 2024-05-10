import streamlit as st
import pandas as pd

from auth import check_password


def show_book(id):
    query = 'SELECT author, title FROM books where id = \'' + id + '\';'
    # print(query)
    df = conn.query(
        query, ttl="10m"
    )

    st.dataframe(df, hide_index=True, use_container_width=True)

    # # Print results.
    for row in df.itertuples():
        st.write(f"{row.author} wrote {row.title}")


st.title("Book page!")

# Initialize connection.
conn = st.connection("postgresql", type="sql")

if not check_password():
    st.stop()


id = st.text_input("Show me book number ", value=1)
if st.button("Go!"):
    show_book(id)
