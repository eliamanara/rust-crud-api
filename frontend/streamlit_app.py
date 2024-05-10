# streamlit_app.py

import hmac
import pandas as pd
import streamlit as st

from auth import check_password

# Initialize connection.
conn = st.connection("postgresql", type="sql")


if not check_password():
    st.stop()


# Perform query.
df = conn.query('SELECT author, title FROM books;', ttl="10m")

st.dataframe(df, hide_index=True, use_container_width=True)

# # Print results.
# for row in df.itertuples():
#     st.write(f"{row.author} wrote {row.title}")
