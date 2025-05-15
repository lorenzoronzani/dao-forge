import React from 'react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import TopBar from '@/components/headers/TopBar';
import { MainContainer } from '@/layouts/MainContainer';
import { BackButton } from '@/components/buttons/BackButton';
import { useNavigate, useParams } from 'react-router';
import { useDao } from '@/providers/DaoProvider';
import { Principal } from '@dfinity/principal';
import { DaoInformation } from '@/views/dao-details/DaoInformation';
import { MembersView } from '@/views/dao-details/MembersView';
import { DocumentsView } from '@/views/dao-details/DocumentsView';
import { PublicationsView } from '@/views/dao-details/PublicationsView';
import { VotationsView } from '@/views/dao-details/VotationsView';

export const DaoDetailsPage = () => {
    const navigate = useNavigate();
    const { id } = useParams();
    const { getDao } = useDao();

    const dao = getDao(Principal.fromText(id!));

    return (
        <>
            <TopBar />

            <MainContainer>
                <BackButton onBack={() => navigate(-1)} />

                <DaoInformation dao={dao} />

                <Tabs defaultValue="members" className="mt-6">
                    <TabsList className="mb-8 bg-white p-1 shadow-sm">
                        <TabsTrigger value="members">Members</TabsTrigger>
                        <TabsTrigger value="votations">Votations</TabsTrigger>
                        <TabsTrigger value="publications">Publications</TabsTrigger>
                        <TabsTrigger value="documents">Documents</TabsTrigger>
                    </TabsList>

                    <TabsContent value="members">
                        <MembersView dao={dao} />
                    </TabsContent>

                    <TabsContent value="votations">
                        <VotationsView votations={[]} />
                    </TabsContent>

                    <TabsContent value="publications">
                        <PublicationsView />
                    </TabsContent>

                    <TabsContent value="documents">
                        <DocumentsView documents={dao.documents}/>
                    </TabsContent>
                </Tabs>
            </MainContainer>
        </>
    );
};

export default DaoDetailsPage;