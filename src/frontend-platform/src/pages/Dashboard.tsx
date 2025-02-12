import { DashboardHeader } from '@/components/headers/DashboardHeader';
import { PaddedLayout } from '@/layouts/PaddedLayout';
import { DashboardGrid } from '@/components/grids/DashboardGrid';
import { useState } from 'react';
import { DaoAssociation } from '@/declarations/dao/dao-association.types';

export const Dashboard = () => {
    const [daos, setDaos] = useState<DaoAssociation[]>([]);

    return (
        <PaddedLayout>
            <DashboardHeader />
            <DashboardGrid daos={daos} />
        </PaddedLayout>
    );
};