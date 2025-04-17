import { useAuthentication } from '@/providers/AuthenticationProvider';
import TopBar from '@/components/headers/TopBar';
import { LoggedInView } from '@/views/dashboard/LoggedInView';
import { LoggedOutView } from '@/views/dashboard/LoggedOutView';
import { Dao, LegalForm, OrganizationStatus } from '@/declarations/common';

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

export const exploreDaos: Dao[] = [
    {
        name: 'ClimateDAO',
        address: 'Bahnhofstrasse 25',
        zip: 8001,
        town: 'Zurich',
        legal_form: LegalForm.Association,
        status: OrganizationStatus.Active,
        uid: 'CHE-123.456.789',
        ch_id: 'CH0123456789',
        frc_id: 12345,
        purpose: 'A DAO focused on funding climate change initiatives',
        sogc_pubblications: [20230401, 20220615],
        board: ['John Doe', 'Jane Smith', 'Alex Johnson'],
        members: ['member1', 'member2', 'member3', 'member4', 'member5'],
        created_at: 1609459200000  // Jan 1, 2021
    },
    {
        name: 'DevFund',
        address: 'Langstrasse 45',
        zip: 8005,
        town: 'Zurich',
        legal_form: LegalForm.Corporation,
        status: OrganizationStatus.Liquidation,
        uid: 'CHE-987.654.321',
        ch_id: 'CH9876543210',
        frc_id: 54321,
        purpose: 'Supporting open source developers on Internet Computer',
        sogc_pubblications: [20230215],
        board: ['Sarah Williams', 'Michael Brown'],
        members: ['member1', 'member2', 'member3', 'member4', 'member5', 'member6', 'member7'],
        created_at: 1625097600000  // Jul 1, 2021
    },
    {
        name: 'ArtCollective',
        address: 'Via Nassa 15',
        zip: 6900,
        town: 'Lugano',
        legal_form: LegalForm.LimitedLiabilityCompany,
        status: OrganizationStatus.Dissolved,
        uid: 'CHE-456.789.123',
        ch_id: 'CH4567891230',
        frc_id: 67890,
        purpose: 'Digital art curation and investments',
        sogc_pubblications: [],
        board: ['Laura Rossi', 'Marco Bianchi', 'Elena Verdi'],
        members: ['member1', 'member2', 'member3'],
        created_at: 1640995200000  // Jan 1, 2022
    },
];

export const userDaos: Dao[] = [
    {
        name: 'My Project DAO',
        address: 'Binzallee 32',
        zip: 8055,
        town: 'Zurich',
        legal_form: LegalForm.LimitedLiabilityCompany,
        status: OrganizationStatus.Active,
        uid: 'CHE-111.222.333',
        ch_id: 'CH1112223330',
        frc_id: 11122,
        purpose: 'Your personal project funding organization',
        sogc_pubblications: [20230515, 20230701],
        board: ['Your Name'],
        members: ['member1', 'member2'],
        created_at: 1672531200000  // Jan 1, 2023
    },
    {
        name: 'Community Initiative',
        address: 'Seestrasse 120',
        zip: 8002,
        town: 'Zurich',
        legal_form: LegalForm.Association,
        status: OrganizationStatus.Active,
        uid: 'CHE-444.555.666',
        ch_id: 'CH4445556660',
        frc_id: 44556,
        purpose: 'Local community governance structure',
        sogc_pubblications: [20230901],
        board: ['Community Leader', 'Your Name'],
        members: ['member1', 'member2', 'member3', 'member4'],
        created_at: 1680307200000  // Apr 1, 2023
    },
];