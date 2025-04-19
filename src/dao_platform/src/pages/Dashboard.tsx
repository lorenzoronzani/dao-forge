import { useAuthentication } from '@/providers/AuthenticationProvider';
import TopBar from '@/components/headers/TopBar';
import { LoggedInView } from '@/views/dashboard/LoggedInView';
import { LoggedOutView } from '@/views/dashboard/LoggedOutView';

export const Dashboard = () => {
    const { isAuthenticated } = useAuthentication();

    return (
        <>
            <TopBar />

            <main className="container mx-auto px-4 py-8">
                {isAuthenticated ? (
                    <LoggedInView />
                ) : (
                    <LoggedOutView />
                )}
            </main>
        </>
    );
};