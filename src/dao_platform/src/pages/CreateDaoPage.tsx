import TopBar from "@/components/headers/TopBar";
import { MainContainer } from "@/layouts/MainContainer";
import { BackButton } from "@/components/buttons/BackButton";
import { DaoForm } from '@/components/forms/DaoForm';
import { useNavigate } from "react-router";
import { DaoAgencyService } from '@/services/daoAgencyService';
import { ICP_CANISTER_ID } from '@/constants/icp';
import { DaoAssociationInitArgs } from '../../../declarations/dao_agency/dao_agency.did.js';
import { Principal } from "@dfinity/principal";
import { useAuthentication } from "@/providers/AuthenticationProvider.js";
import { useDao } from "@/providers/DaoProvider.js";
import { toast } from "@/hooks/use-toast.js";

export const CreateDaoPage = () => {
    const { identity } = useAuthentication();
    const navigate = useNavigate();
    const { refreshData } = useDao();

    const onSubmit = async (dao: DaoAssociationInitArgs): Promise<Principal> => {
        const daoAgencyService = new DaoAgencyService(ICP_CANISTER_ID.DAO_AGENCY, identity);

        try {
            const principal = await daoAgencyService.createDaoAssociation(dao);

            refreshData();

            return principal;
        } catch (error) {
            toast({
                title: "Error!",
                description: "An error occurred while creating the DAO.",
                duration: 2000,
            });

            throw error;
        }
    }

    return (
        <>
            <TopBar />

            <MainContainer>
                <BackButton onBack={() => navigate(-1)} />

                <div className="max-w-3xl mx-auto">
                    <h1 className="text-2xl font-bold mb-6">Create New DAO</h1>
                    <p className="text-slate-600 mb-8">
                        Fill in the details to create your new decentralized autonomous organization.
                    </p>

                    <DaoForm onSubmit={onSubmit} onCancel={() => navigate(-1)} />
                </div>
            </MainContainer>
        </>
    );
}