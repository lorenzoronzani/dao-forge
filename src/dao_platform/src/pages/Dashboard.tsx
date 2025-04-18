import { useAuthentication } from '@/providers/AuthenticationProvider';
import TopBar from '@/components/headers/TopBar';
import { LoggedInView } from '@/views/dashboard/LoggedInView';
import { LoggedOutView } from '@/views/dashboard/LoggedOutView';
import { DaoDiscoveryService } from '@/services/daoDiscoveryService';
import { useQuery } from '@tanstack/react-query';
import { DaoAssociation } from '@/models/entities/DaoAssociation';
import { LegalForm, OrganizationStatus } from '@/models/entities/Dao';

export const Dashboard = () => {
    const { isAuthenticated } = useAuthentication();

    // const daoDiscoveryService = new DaoDiscoveryService();

    // const daos = useQuery<unknown[]>({
    //     queryKey: ['daos'],
    //     queryFn: () => daoDiscoveryService.getRandomDaos()
    // });

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

export const exploreDaos: DaoAssociation[] = [
    new DaoAssociation(
        'ClimateDAO',
        'Bahnhofstrasse 25',
        8001,
        'Zurich',
        LegalForm.Association,
        OrganizationStatus.Active,
        'CHE-123.456.789',
        'CH0123456789',
        12345,
        'A DAO focused on funding climate change initiatives',
        [],
        [],
        [],
        new Date(1609459200000)  // Jan 1, 2021
    ),
];