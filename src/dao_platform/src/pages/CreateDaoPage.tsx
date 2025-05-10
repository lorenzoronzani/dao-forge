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
import { PdfFormFieldType, PdfFormFillData, PdfService } from "@/services/pdfService.js";
import { DaoFormData } from '@/components/forms/DaoForm';
import { ASSOCIATION_NOTIFICATION_FORM } from "@/constants/pdf/association-form.js";
import { formatDate } from "@/utils/date.js";

export const CreateDaoPage = () => {
    const { identity, userPrincipal } = useAuthentication();
    const navigate = useNavigate();
    const { refreshData } = useDao();

    const onSubmitAssociation = async (formData: DaoFormData): Promise<Principal> => {
        const daoAgencyService = new DaoAgencyService(ICP_CANISTER_ID.DAO_AGENCY, identity);

        const daoAssociationInitArgs: DaoAssociationInitArgs = {
            name: formData.name,
            address: formData.address,
            zip: formData.zip,
            town: formData.town,
            uid: `CHE-${Math.floor(Math.random() * 1000)}.${Math.floor(Math.random() * 1000)}.${Math.floor(Math.random() * 1000)}`,
            ch_id: `CH${Math.floor(Math.random() * 10000000000)}`,
            frc_id: Math.floor(Math.random() * 100000),
            purpose: formData.purpose,
            board: formData.boardMembers.map(b => Principal.fromText(b)),
            members: formData.members.map(m => Principal.fromText(m))
        };

        try {
            const principal = await daoAgencyService.createDaoAssociation(daoAssociationInitArgs);

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

    const onSubmitPdfGeneration = async (formData: DaoFormData): Promise<void> => {
        const data: PdfFormFillData[] = [
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.SENDER.FULL_NAME,
                value: formData.userFirstName + " " + formData.userLastName
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.SENDER.FIRM,
                value: "Dao Forge"
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.SENDER.ADDRESS,
                value: formData.userAddress
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.SENDER.ZIP_CODE_AND_LOCALITY,
                value: formData.userZip.toString()
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.SENDER.PHONE,
                value: formData.userPhone
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.SENDER.EMAIL,
                value: formData.userEmail
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.NAME.NAME,
                value: formData.name
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.NAME.UID,
                value: `CHE-${Math.floor(Math.random() * 1000)}.${Math.floor(Math.random() * 1000)}.${Math.floor(Math.random() * 1000)}`
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.PURPOSE.PURPOSE,
                value: formData.purpose
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.STATUTE.CREATION_DATE,
                value: formatDate(Date.now())
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.STATUTE.STATUTE_DATE,
                value: formatDate(Date.now())
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.PURPOSE.PURPOSE,
                value: formData.purpose
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.HEADQUARTERS.POLITICAL_LOCALITY,
                value: formData.town
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.HEADQUARTERS.LEGAL_ADDRESS,
                value: formData.address
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.HEADQUARTERS.LEGAL_ZIP_CODE_AND_LOCALITY,
                value: formData.zip.toString()
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.RESPONSIBILITY.HOW,
                value: ''
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.PEOPLE_TO_ENROLL.NUMBER,
                value: formData.boardMembers.length.toString()
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_FORM.FIELDS.SIGNATURE.SIGNATURE_LOCATION_AND_DATE,
                value: formatDate(Date.now())
            },
        ];

        const pdfBytes = await PdfService.fill(ASSOCIATION_NOTIFICATION_FORM.TEMPLATE_URL, data);

        PdfService.openPdf(pdfBytes);
    }

    const onSubmit = async (formData: DaoFormData): Promise<Principal> => {
        const principal = await onSubmitAssociation(formData);

        await onSubmitPdfGeneration(formData);

        return principal;
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

                    <DaoForm onSubmit={onSubmit} onCancel={() => navigate(-1)} userPrincipal={userPrincipal} />
                </div>
            </MainContainer>
        </>
    );
}