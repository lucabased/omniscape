Queue (A queue ready for processing):
    HashMap<String, bool>
        - String = Sec-UID
        - bool = Is unscrapable? (e.g. is_private or is_secret)

Already_Checked:
    HashMap<String, bool>
        - String = Sec-UID
        - bool = Has been checked?