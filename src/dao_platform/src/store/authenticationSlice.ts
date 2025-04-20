import { Principal } from '@dfinity/principal'
import { createSlice, PayloadAction } from '@reduxjs/toolkit'

interface AuthenticationState {
    isAuthenticated: boolean;
    userPrincipal: string;
}

const initialState: AuthenticationState = {
    isAuthenticated: false,
    userPrincipal: Principal.anonymous().toText()
}

const authenticationSlice = createSlice({
    name: 'authentication',
    initialState,
    reducers: {
        login: (state, action: PayloadAction<string>) => {
            state.isAuthenticated = true;
            state.userPrincipal = action.payload;
        },
        logout: state => {
            state.isAuthenticated = false;
            state.userPrincipal = Principal.anonymous().toText();
        }
    }
})

export const { login, logout } = authenticationSlice.actions;
export default authenticationSlice.reducer;