import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Download, Eye, File, Loader2 } from "lucide-react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { formatDate } from "@/utils/date";
import { Button } from "@/components/ui/button";
import { PdfService } from "@/services/pdfService";
import { useRef } from "react";
import { useAuthentication } from "@/providers/AuthenticationProvider";
import { useDao } from "@/providers/DaoProvider";
import { DocumentsStorageService } from "@/services/documentsStorageService";
import { ICP_CANISTER_ID } from "@/constants/icp";
import { DocumentArgs } from "declarations/documents_storage/documents_storage.did.js";
import { useState } from "react";
import { toast } from "@/hooks/use-toast";
import { DaoAssociationService } from "@/services/daoAssocationService";
import { Dao } from "@/models/entities/Dao";


interface DocumentsViewProps {
    dao: Dao;
}

export const DocumentsView = ({ dao }: DocumentsViewProps) => {
    const { identity } = useAuthentication();
    const { refreshData } = useDao();
    const fileInputRef = useRef<HTMLInputElement>(null);
    const [isUploading, setIsUploading] = useState(false);

    const documents = dao.documents;

    const handleUploadClick = () => {
        fileInputRef.current?.click();
    };

    const updateDao = async (documentId: number) => {
        const daoService = new DaoAssociationService(dao.principal, identity);

        try {
            await daoService.addDocument(documentId);
        } catch (error) {
            toast({
                title: "Error!",
                description: "An error occurred while updating the DAO.",
                duration: 2000,
            });

            throw error;
        }
    }

    const handleFileChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0];
        if (!file) {
            return;
        }

        setIsUploading(true);
        try {
            const pdfBytes = await file.arrayBuffer();
            const documentStorageService = new DocumentsStorageService(ICP_CANISTER_ID.DOCUMENTS_STORAGE, identity);

            const documentArgs: DocumentArgs = {
                content: new Uint8Array(pdfBytes),
                name: file.name,
                content_type: file.type
            };

            const documentId = await documentStorageService.uploadDocument(documentArgs);

            await updateDao(documentId);

            refreshData();

            toast({
                title: "Success!",
                description: "Document uploaded successfully.",
                duration: 2000,
            });
        } catch (error) {
            toast({
                title: "Error!",
                description: "An error occurred while uploading the document.",
                duration: 2000,
            });
        } finally {
            setIsUploading(false);
        }
    };

    return (
        <Card>
            <CardHeader>
                <div className="flex items-center justify-between">
                    <CardTitle>Documents</CardTitle>
                    <Button onClick={handleUploadClick} disabled={isUploading}>{isUploading ? (
                        <>
                            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                            Uploading...
                        </>
                    ) : (
                        "Upload document"
                    )}
                    </Button>
                </div>
                <CardDescription>Important DAO documents and files</CardDescription>
            </CardHeader>
            <CardContent>
                {documents && documents.length > 0 ? (
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>Name</TableHead>
                                <TableHead>Owner</TableHead>
                                <TableHead>Updated</TableHead>
                                <TableHead>Actions</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {documents.map((document) => (
                                <TableRow key={document.id}>
                                    <TableCell className="font-medium">{document.name}</TableCell>
                                    <TableCell className="font-mono text-xs truncate max-w-[150px]" title={document.owner.toText()}>
                                        {document.owner.toText()}
                                    </TableCell>
                                    <TableCell>{formatDate(document.updatedAt.getTime())}</TableCell>
                                    <TableCell>
                                        <div className="flex items-center gap-2">
                                            <Button variant="ghost" size="sm" onClick={() => PdfService.downloadPdf(document.name, document.content)}>
                                                <Download className="h-4 w-4" />
                                            </Button>
                                            <Button variant="ghost" size="sm" onClick={() => PdfService.openPdf(document.content)}>
                                                <Eye className="h-4 w-4" />
                                            </Button>
                                        </div>
                                    </TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                ) : (
                    <div className="text-center py-8 text-slate-500">
                        <File className="h-12 w-12 mx-auto mb-4 text-slate-300" />
                        <h3 className="font-medium mb-1">No Documents Yet</h3>
                        <p>There are no documents available for this DAO.</p>
                        <Button onClick={handleUploadClick} disabled={isUploading}>{isUploading ? (
                            <>
                                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                                Uploading...
                            </>
                        ) : (
                            "Upload document"
                        )}
                        </Button>
                    </div>
                )}
            </CardContent>
            <input
                type="file"
                ref={fileInputRef}
                onChange={handleFileChange}
                className="hidden"
                accept="application/pdf"
            />
        </Card>
    );
};