import { useAuthentication } from '@/providers/AuthenticationProvider';
import TopBar from '@/components/headers/TopBar';
import { LoggedInView } from '@/views/dashboard/LoggedInView';
import { LoggedOutView } from '@/views/dashboard/LoggedOutView';

export const Dashboard = () => {
    const { isAuthenticated } = useAuthentication();

    // const daoDiscoveryService = new DaoDiscoveryService();

    // const daos = useQuery<unknown[]>({
    //     queryKey: ['daos'],
    //     queryFn: () => daoDiscoveryService.getDaos(userPrincipal)
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

export interface DAO {
    id: string;
    name: string;
    description: string;
    members: number;
    assets: string;
}

export const exploreDaos: DAO[] = [
    {
        id: '1',
        name: 'ClimateDAO',
        description: 'A DAO focused on funding climate change initiatives',
        members: 1240,
        assets: '520,000 ICP'
    },
    {
        id: '2',
        name: 'DevFund',
        description: 'Supporting open source developers on Internet Computer',
        members: 890,
        assets: '320,000 ICP'
    },
    {
        id: '3',
        name: 'ArtCollective',
        description: 'Digital art curation and investments',
        members: 450,
        assets: '180,000 ICP'
    },
];

export const userDaos: DAO[] = [
    {
        id: '4',
        name: 'My Project DAO',
        description: 'Your personal project funding organization',
        members: 12,
        assets: '5,000 ICP'
    },
    {
        id: '5',
        name: 'Community Initiative',
        description: 'Local community governance structure',
        members: 48,
        assets: '12,000 ICP'
    },
];