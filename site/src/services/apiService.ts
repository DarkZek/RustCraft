import type { AxiosResponse } from 'axios'
import { axios } from './axios'

export async function isActive(): Promise<boolean> {
    try {
        await axios.get(
            '/'
        )
        return true
    } catch (e) {
        return false
    }
}

export async function login(username: string): Promise<AxiosResponse<{ refresh_token: string }>> {
    return await axios.post(
        '/login',
        { username }
    )
}