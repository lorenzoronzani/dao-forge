import { useAuthentication } from '@/providers/AuthenticationProvider';
import TopBar from '@/components/headers/TopBar';
import { LoggedInView } from '@/views/dashboard/LoggedInView';
import { LoggedOutView } from '@/views/dashboard/LoggedOutView';
import { MainContainer } from '@/layouts/MainContainer';

export const Dashboard = () => {
    const { isAuthenticated } = useAuthentication();

    return (
        <>
            <TopBar />

            <MainContainer>
                {isAuthenticated ? (
                    <LoggedInView />
                ) : (
                    <LoggedOutView />
                )}
            </MainContainer>
        </>
    );
};